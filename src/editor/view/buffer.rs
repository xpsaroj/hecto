use super::Location;
use super::line::Line;
use std::fs;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let file_contents = fs::read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, character: char, at: Location) {
        // if 2 lines down, skip that.
        if at.line_index > self.lines.len() {
            return;
        }

        // if 1 line down, just add a new line with that char, else let Line handle insertion (if in any other line)
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            // if at the end of one line and next line exists, on delete, append the line below it to this one. else just do a normal delete
            if at.grapheme_index >= line.grapheme_count()
                && self.lines.len() > at.line_index.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));

                // this line exists cause of that outer if let check
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }
}
