use std::{ops::Range, result};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            fragments: Self::str_to_fragments(line_str),
        }
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect()
    }

    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ => {
                let mut chars = for_str.chars();
                match (chars.next(), chars.next()) {
                    // Exactly one control character
                    (Some(ch), None) if ch.is_control() => Some('▯'),

                    // Exactly one non-control character (zero-width)
                    (Some(_), None) if width == 0 => Some('·'),

                    // Multiple characters or normal text → unchanged
                    _ => None,
                }
            }
        }
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break;
            }
            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = fragment_end;
        }

        result
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn insert_char(&mut self, character: char, grapheme_index: usize) {
        // create a new str and convert it into Vec<TextFragment>
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            // If in between somewhere, add chars before it, then the new char and then everything after it.
            if index == grapheme_index {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }

        // If at the end of line, just add the char
        if grapheme_index >= self.fragments.len() {
            result.push(character);
        }

        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete(&mut self, grapheme_index: usize) {
        let mut result = String::new();

        // skip the one at the index, add everything else
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index != grapheme_index {
                result.push_str(&fragment.grapheme);
            }
        }

        self.fragments = Self::str_to_fragments(&result);
    }
}
