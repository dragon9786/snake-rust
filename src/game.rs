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
}

impl Board {
    pub fn new(height: u16, width: u16) -> Self {
        Board { height, width }
    }

    pub fn randomPosition(&self) -> Position {
        Position {
            x: rand::thread_rng().gen_range(0..self.width),
            y: rand::thread_rng().gen_range(0..self.height),
        }
    }

    pub fn is_legal(&self, next_position: Position) -> bool {
        next_position.x > 0
            && next_position.x < self.width
            && next_position.y < self.height
            && next_position.y > 0
    }
}
#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub snake: Snake,
}

impl Game {
    pub fn new(height: u16, width: u16) -> Self {
        Game {
            board: Board::new(height, width),
            snake: Snake {
                head: Position { x: 0, y: 0 },
                body: vec![],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: Position,
    pub body: Vec<Position>,
}
