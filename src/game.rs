use crossterm::style::Color;
use crossterm::*;
use crossterm_input::*;
use crossterm_screen::Screen;
use std::io::{stdout, Stdout};
use std::{process, thread, time};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub height: u16,
    pub width: u16,
    pub food: Position,
    pub snake: Snake,
}

impl Board {
    pub fn new(height: u16, width: u16) -> Self {
        let snake_start = random_position(width, height);
        let food_start = random_position(width, height);

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

    pub fn is_legal(&self, next_position: Position) -> bool {
        next_position.x > 0
            && next_position.x < self.width
            && next_position.y < self.height
            && next_position.y > 0
    }

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
                        .queue(crossterm::cursor::MoveTo(x, y))
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
                        .queue(crossterm::cursor::MoveTo(x, y))
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
        // Place the food at it's position by drawing it
        stdout
            .queue(crossterm::cursor::DisableBlinking)
            .unwrap()
            .queue(crossterm::cursor::Hide)
            .unwrap()
            .queue(crossterm::cursor::MoveTo(self.food.x, self.food.y))
            .unwrap()
            .queue(crossterm::style::SetColors(crossterm::style::Colors::new(
                Color::DarkYellow,
                Color::Black,
            )))
            .unwrap()
            .queue(crossterm::style::Print("@"))?;

        stdout
            .queue(crossterm::cursor::MoveTo(
                self.snake.head.x,
                self.snake.head.y,
            ))
            .unwrap()
            .queue(crossterm::cursor::Show)
            .unwrap()
            .queue(crossterm::cursor::EnableBlinking)
            .unwrap();

        Ok(())
    }
}

pub fn random_position(width: u16, height: u16) -> Position {
    Position {
        x: rand::thread_rng().gen_range(1..width - 1),
        y: rand::thread_rng().gen_range(1..height - 1),
    }
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub score: u16,
}

impl Game {
    pub fn new(height: u16, width: u16) -> Self {
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
                            self.board.snake.grow(&Direction::Up, stdout)
                        }

                        KeyEvent::Down => {
                            game_started = true;
                            self.board.snake.grow(&Direction::Down, stdout)
                        }
                        KeyEvent::Right => {
                            game_started = true;
                            self.board.snake.grow(&Direction::Right, stdout)
                        }
                        KeyEvent::Left => {
                            game_started = true;
                            self.board.snake.grow(&Direction::Left, stdout)
                        }
                        KeyEvent::Char('q') => {
                            stdin.stop_reading();
                            self.quit(stdout)?;
                        }
                        _ => {
                            if game_started {
                                self.board.snake.grow(self.board.snake.direction, stdout);
                            }
                        }
                    },
                    _ => {}
                }
            } else {
                if game_started {
                    std::thread::sleep(time::Duration::from_millis(600));
                    self.board.snake.grow(self.board.snake.direction, stdout);
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
                // add food in a new random position and continue
                self.board.food = random_position(self.board.width - 1, self.board.height - 1);
                self.score = self.score + 1;
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

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: Position,
    pub direction: &'static Direction,
    pub body: Vec<Position>,
}

impl Snake {
    pub fn at_head(&self, x: u16, y: u16) -> Option<&'static str> {
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

    pub fn grow(&mut self, direction: &'static Direction, stdout: &mut Stdout) {
        match direction {
            Direction::Up => {
                self.head.y = self.head.y - 1;
            }
            Direction::Down => {
                self.head.y = self.head.y + 1;
            }
            Direction::Left => {
                self.head.x = self.head.x - 1;
            }
            Direction::Right => {
                self.head.x = self.head.x + 1;
            }
        }
        self.direction = direction;
    }

    pub fn collided_with_wall(&mut self, x: u16, y: u16) -> bool {
        self.head.x == 0 || self.head.y == 0 || self.head.y == y - 1 || self.head.x == x - 1
    }
}
