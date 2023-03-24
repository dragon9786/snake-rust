#![allow(unused_imports, dead_code)]
mod game;

use crate::game::Game;
use crossterm::*;
use rand::Rng;
use std::io::stdout;
use crossterm_screen::Screen;
use crossterm_input::*;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut game = Game::new(24, 24);

    stdout().execute(crossterm::terminal::Clear(terminal::ClearType::All))?;
    let screen = Screen::new(true);
    let mut input = TerminalInput::from_output(&screen.stdout);
    let mut stdin = input.read_async();
    loop {
        let _ = game.tick(&mut stdin);
    }


    // Commented out to let your cpu take a break
    // loop {
    //     game.tick()
    // }
}
