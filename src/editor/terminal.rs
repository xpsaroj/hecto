use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};

use core::fmt::Display;
use std::io::{Error, Write, stdout};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

/// Represents the Terminal.
/// 
/// Edge Case for platforms where `usize`  < `u16`:
/// 
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller
///
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`).
/// And should you attempt to set the caret out of these bounds, it will also be truncated.
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
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

    /// Moves the caret to the given Position
    /// * `Position` - the `Position` to move the caret to. Will be truncated to `u16::MAX` if bigger
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        let height = height as usize;
        let width = width as usize;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}
