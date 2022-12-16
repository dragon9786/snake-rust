#![allow(unused_imports, dead_code)]
mod game;

use crate::game::Game;
use crossterm::*;
use rand::Rng;
use std::io::stdout;

fn main() -> Result<()> {
    println!("Hello, world!");
    let game = Game::new(12, 12);

    stdout().execute(crossterm::terminal::Clear(terminal::ClearType::All))?;

    for y in 0..game.board.height {
        for x in 0..game.board.width {
            stdout()
                .queue(crossterm::cursor::MoveTo(x, y))
                .unwrap()
                .queue(crossterm::style::Print("*"))?;
        }
    }

    Ok(())
}
