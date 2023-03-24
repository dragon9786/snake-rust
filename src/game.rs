use std::io::stdout;
use std::{thread, time};
use crossterm::*;
use crossterm_screen::Screen;
use crossterm_input::*;


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

        Board { height, width, snake: Snake {
                head: snake_start,
            body: vec![],
            direction: Direction::Up,
        },
                food: food_start,
        }
    }

    pub fn random_position(&self) -> Position {
        random_position(self.width, self.height)
    }

    pub fn is_legal(&self, next_position: Position) -> bool {
        next_position.x > 0
            && next_position.x < self.width
            && next_position.y < self.height
            && next_position.y > 0
    }

    pub fn draw(&self) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                // Check if this is interesting, if so draw it
                if let Some(direction) = self.snake.at_head(x,y){
                    stdout()
                        .queue(crossterm::cursor::MoveTo(x, y))?
                        .queue(crossterm::style::Print(direction))?;
                }

                if y == 0 ||
                    y == (self.height-1) ||
                    x == 0 ||
                    x ==  (self.width-1) {


                        stdout()
                            .queue(crossterm::cursor::MoveTo(x, y))
                            .unwrap()
                            .queue(crossterm::style::Print("*")).unwrap();
                    }
            }
        }
        Ok(())
    }
}

pub fn random_position(width: u16, height: u16) -> Position {
        Position {
            x: rand::thread_rng().gen_range(0..width),
            y: rand::thread_rng().gen_range(0..height),
        }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new(height: u16, width: u16) -> Self {
        Game {
            board: Board::new(height, width),
        }
    }
    pub fn tick(&mut self) -> Result<()> {
        // Take player input
        // check if it's legal/game is over
        // Draw board
        // Go back to start
        let screen = Screen::new(true);
        let mut input = TerminalInput::from_output(&screen.stdout);
        let mut stdin = input.read_async();
        if let Some(key_event) = stdin.next() {
            match key_event {
                InputEvent::Keyboard(key_event) => match key_event {
                    // KeyEvent::Up => self.board.snake.direction = Direction::Up,
                    // KeyEvent::Up => self.board.snake.direction = Direction::Up,
                    KeyEvent::Up => {
                        screen.stdout.write_string(format!("Got to the up \n")).unwrap();
                    },
                    KeyEvent::Down => self.board.snake.direction = Direction::Down,
                    KeyEvent::Right => self.board.snake.direction = Direction::Right,
                    KeyEvent::Left => self.board.snake.direction = Direction::Left,
                    _ => {},
                },
                _ => {
                    screen.stdout.write_string(format!("Got to the default arm\n")).unwrap();
                },
            }
        }
        thread::sleep(time::Duration::from_millis(1000));
        self.board.draw()

    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: Position,
    pub direction: Direction,
    pub body: Vec<Position>,
}

impl Snake {
    pub fn at_head(&self, x:u16, y:u16) -> Option<&'static str> {
        if self.head.x == x && self.head.y == y {
            return match &self.direction {
                Direction::Up => Some("^"),
                Direction::Down => Some("v"),
                Direction::Right => Some(">"),
                Direction::Left => Some("<"),
            }
        }
        return None
    }
}
