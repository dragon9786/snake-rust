use crossterm::style::Color;
use crossterm::*;
use crossterm_input::*;
use crossterm_screen::Screen;
use std::io::{stdout, Stdout};
use std::ops::Add;
use std::{process, thread, time};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone)]
pub struct Position {
    point: &'static Point,
    direction: &'static Direction,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub height: i16,
    pub width: i16,
    pub food: Point,
    pub snake: Snake,
}

impl Add for Point {
    type Output = Point;
    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

impl Board {
    pub fn new(height: i16, width: i16) -> Self {
        let snake_start = random_Point(width, height);
        let food_start = random_Point(width, height);

        Board {
            height,
            width,
            snake: Snake {
                head: snake_start,
                body: vec![],
                direction: &Direction::Up,
            },
            food: food_start,
        }
    }

    // pub fn is_legal(&self, next_Point: Point) -> bool {
    //     next_Point.x > 0
    //         && next_Point.x < self.width
    //         && next_Point.y < self.height
    //         && next_Point.y > 0
    // }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                // Check if this is interesting, if so draw it
                if let Some(direction) = self.snake.at_head(x, y) {
                    stdout
                        .queue(crossterm::cursor::DisableBlinking)
                        .unwrap()
                        .queue(crossterm::cursor::Hide)
                        .unwrap()
                        .queue(crossterm::cursor::MoveTo(x as u16, y as u16))
                        .unwrap()
                        .queue(crossterm::style::SetColors(crossterm::style::Colors::new(
                            Color::DarkGreen,
                            Color::Black,
                        )))
                        .unwrap()
                        .queue(crossterm::style::Print(direction))?;
                }

                if y == 0 || y == (self.height - 1) || x == 0 || x == (self.width - 1) {
                    stdout
                        .queue(crossterm::cursor::DisableBlinking)
                        .unwrap()
                        .queue(crossterm::cursor::Hide)
                        .unwrap()
                        .queue(crossterm::cursor::MoveTo(x as u16, y as u16))
                        .unwrap()
                        .queue(crossterm::style::SetColors(crossterm::style::Colors::new(
                            Color::DarkRed,
                            Color::Red,
                        )))
                        .unwrap()
                        .queue(crossterm::style::Print(" "))?;
                }
            }
        }
        // Place the food at it's Point by drawing it
        stdout
            .queue(crossterm::cursor::DisableBlinking)
            .unwrap()
            .queue(crossterm::cursor::Hide)
            .unwrap()
            .queue(crossterm::cursor::MoveTo(
                self.food.x as u16,
                self.food.y as u16,
            ))
            .unwrap()
            .queue(crossterm::style::SetColors(crossterm::style::Colors::new(
                Color::DarkYellow,
                Color::Black,
            )))
            .unwrap()
            .queue(crossterm::style::Print("@"))?;

        stdout
            .queue(crossterm::cursor::MoveTo(
                self.snake.head.x as u16,
                self.snake.head.y as u16,
            ))
            .unwrap()
            .queue(crossterm::cursor::Show)
            .unwrap()
            .queue(crossterm::cursor::EnableBlinking)
            .unwrap();

        Ok(())
    }
}

pub fn random_Point(width: i16, height: i16) -> Point {
    Point {
        x: rand::thread_rng().gen_range(1..width - 1),
        y: rand::thread_rng().gen_range(1..height - 1),
    }
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub score: i16,
}

impl Game {
    pub fn new(height: i16, width: i16) -> Self {
        Game {
            board: Board::new(height, width),
            score: 0,
        }
    }

    pub fn tick(&mut self, stdin: &mut AsyncReader, stdout: &mut Stdout) -> Result<()> {
        // Take player input
        // check if it's legal/game is over
        // Draw board
        // Go back to start
        let mut game_started = false;
        self.board.draw(stdout)?;
        loop {
            std::thread::sleep(time::Duration::from_millis(600));
            if let Some(key_event) = stdin.next() {
                match key_event {
                    InputEvent::Keyboard(k) => match k {
                        KeyEvent::Up => {
                            game_started = true;
                            self.board.snake.slither(&Direction::Up, stdout)
                        }

                        KeyEvent::Down => {
                            game_started = true;
                            self.board.snake.slither(&Direction::Down, stdout)
                        }
                        KeyEvent::Right => {
                            game_started = true;
                            self.board.snake.slither(&Direction::Right, stdout)
                        }
                        KeyEvent::Left => {
                            game_started = true;
                            self.board.snake.slither(&Direction::Left, stdout)
                        }
                        KeyEvent::Char('q') => {
                            stdin.stop_reading();
                            self.quit(stdout)?;
                        }
                        _ => {
                            if game_started {
                                self.board.snake.slither(self.board.snake.direction, stdout);
                            }
                        }
                    },
                    _ => {}
                }
            } else {
                if game_started {
                    std::thread::sleep(time::Duration::from_millis(600));
                    self.board.snake.slither(self.board.snake.direction, stdout);
                }
            }

            if self
                .board
                .snake
                .collided_with_wall(self.board.height, self.board.width)
            {
                self.quit(stdout)?;
            }
            if self.board.food == self.board.snake.head {
                // add food in a new random Point and continue
                self.board.food =
                    random_Point(self.board.width as i16 - 1, self.board.height as i16 - 1);
                self.score = self.score + 1;
                self.board.snake.grow_body();
            }
            self.board.draw(stdout)?;
        }
    }

    pub fn quit(&mut self, stdout: &mut Stdout) -> Result<()> {
        let score_message = format!("\n\n Game Over!!! Total score is :{:?} \n", self.score);
        stdout.execute(crossterm::terminal::Clear(terminal::ClearType::All))?;
        stdout
            .queue(crossterm::style::SetColors(crossterm::style::Colors::new(
                Color::DarkGreen,
                Color::Black,
            )))
            .unwrap()
            .queue(crossterm::style::Print(score_message))?;

        std::process::exit(0);
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn direction_to_Point(&self) -> Point {
        match self {
            Direction::Up => {
                return Point { x: 0, y: -1 };
            }
            Direction::Down => {
                return Point { x: 0, y: 1 };
            }
            Direction::Left => {
                return Point { x: -1, y: 0 };
            }
            Direction::Right => {
                return Point { x: 1, y: 0 };
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: Point,
    pub direction: &'static Direction,
    pub body: Vec<Position>,
}

impl Snake {
    pub fn at_head(&self, x: i16, y: i16) -> Option<&'static str> {
        if self.head.x == x && self.head.y == y {
            return match &self.direction {
                Direction::Up => Some("^"),
                Direction::Down => Some("v"),
                Direction::Right => Some(">"),
                Direction::Left => Some("<"),
            };
        }
        return None;
    }

    pub fn slither(&mut self, direction: &'static Direction, stdout: &mut Stdout) {
        self.head = self.head + Direction::direction_to_Point(direction);
        self.direction = direction
    }

    pub fn get_snake_caboose(&mut self) -> &Point {
        let Point = self.body.get(self.body.len() - 1);
        match Point {
            Some(pos) => return pos.point,
            None => return &self.head,
        }
    }

    pub fn grow_body(&mut self) {
        // Growbody takes the last point from `get_snake_caboose` and the Position
        // Reverse the direction and adds the body to end of Vec<Point>
    }

    pub fn collided_with_wall(&mut self, x: i16, y: i16) -> bool {
        self.head.x == 0
            || self.head.y == 0
            || self.head.y == y as i16 - 1
            || self.head.x == x as i16 - 1
    }
}

// head is always present
// body[0] ->  Point after we first consume some food
//
// match direction {
//     Direction::Up => {
//         self.head.y = self.head.y - 1;
//     }
//     Direction::Down => {
//         self.head.y = self.head.y + 1;
//     }
//     Direction::Left => {
//         self.head.x = self.head.x - 1;
//     }
//     Direction::Right => {
//         self.head.x = self.head.x + 1;
//     }
// }
