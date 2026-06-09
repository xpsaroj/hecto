use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{Command, queue};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

use core::fmt::Display;
use std::io::{Error, Write, stdout};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x, position.y))
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}
