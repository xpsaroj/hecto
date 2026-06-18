mod terminal;

use core::cmp::min;
use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};

use std::io::Error;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => {}
        }

        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => {}
            }
        }
        
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!! \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- v{VERSION}");

        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();

        // Computes self - rhs, saturating at the numeric bounds instead of overflowing. so that padding doesn't wrap to usize::MAX, basically (width - len) / 2 stopping at 0
        let padding = (width.saturating_sub(len)) / 2; // we allow this as we don't care if welcome message is put exactly in the middle, may be little bit left or right (cause int div would be an int, where the actual middle may be a fraction no.)
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
