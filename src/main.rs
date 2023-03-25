#![allow(unused_imports, dead_code)]
mod game;

use crate::game::Game;
use crossterm::*;
use crossterm_input::*;
use crossterm_screen::Screen;
use rand::Rng;
use std::io::stdout;

fn main() -> Result<()> {
    let mut game = Game::new(24, 24);

    stdout().execute(crossterm::terminal::Clear(terminal::ClearType::All))?;
    let screen = Screen::new(true);
    let mut input = TerminalInput::from_output(&screen.stdout);
    input.disable_mouse_mode();
    let mut stdin = input.read_async();
    loop {
        let _ = game.tick(&mut stdin);
    }
}
