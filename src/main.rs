#![allow(unused_imports, dead_code)]
mod game;

use crate::game::Game;
use crossterm::style::Color;
use crossterm::*;
use crossterm_input::*;
use crossterm_screen::Screen;
use rand::Rng;
use std::io::stdout;

fn main() -> Result<()> {
    let mut game = Game::new(24, 24);
    let mut stdout = stdout();
    let screen = Screen::new(true);
    let input = TerminalInput::from_output(&screen.stdout);
    input.disable_mouse_mode()?;
    let mut stdin = input.read_async();

    stdout.execute(crossterm::terminal::Clear(terminal::ClearType::All))?;
    game.board.draw(&mut stdout)?;
    loop {
        let _ = game.tick(&mut stdin, &mut stdout);
    }
}
