#![allow(unused_imports, dead_code)]
mod game;

use crate::game::Game;
use crossterm::*;
use rand::Rng;
use std::io::stdout;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut game = Game::new(24, 24);

    stdout().execute(crossterm::terminal::Clear(terminal::ClearType::All))?;
    loop {
        let _ = game.tick();
    }


    // Commented out to let your cpu take a break
    // loop {
    //     game.tick()
    // }
}
