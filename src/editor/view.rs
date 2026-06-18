use crate::editor::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
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

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
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
        Terminal::print(&welcome_message)
    }
}
