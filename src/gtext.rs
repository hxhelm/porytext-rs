use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GTextLineBreak {
    NewLine,
    NewLineWithScroll,
    NewParagraph,
    End,
}

impl Display for GTextLineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewLine => write!(f, r"\n"),
            Self::NewLineWithScroll => write!(f, r"\l"),
            Self::NewParagraph => write!(f, r"\p"),
            Self::End => write!(f, "$"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GTextEntry {
    pub text: String,
    pub line_break: GTextLineBreak,
}

impl Display for GTextEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t.string \"{}{}\"", self.text, self.line_break)
    }
}

#[derive(Debug)]
pub struct GTextBlock {
    pub name: String,
    pub entries: Vec<GTextEntry>,
}

impl ToString for GTextBlock {
    fn to_string(&self) -> String {
        let mut output = format!("gText_{}::\n", self.name);

        for entry in &self.entries {
            output.push_str(&format!("{entry}\n"));
        }

        output
    }
}

impl GTextBlock {
    pub fn from_plain_text(lines: &[String], name: &str) -> Self {
        let lines = trim_empty_lines(lines);
        let mut entries = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let linebreak;

            if i == lines.len() - 1 {
                linebreak = GTextLineBreak::End;
            } else if i == 0 || lines[i - 1].is_empty() {
                linebreak = if lines[i + 1].is_empty() {
                    GTextLineBreak::NewParagraph
                } else {
                    GTextLineBreak::NewLine
                };
            } else if !lines[i + 1].is_empty() {
                linebreak = GTextLineBreak::NewLineWithScroll;
            } else {
                linebreak = GTextLineBreak::NewParagraph;
            }

            entries.push(GTextEntry {
                text: line.clone(),
                line_break: linebreak,
            });
        }

        if entries.is_empty() {
            entries.push(GTextEntry {
                text: String::new(),
                line_break: GTextLineBreak::End,
            });
        }

        Self {
            name: name.to_string(),
            entries,
        }
    }
}

fn trim_empty_lines(lines: &[String]) -> &[String] {
    let start_index = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(lines.len());
    let end_index = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap_or(0);
    &lines[start_index..=end_index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[String::new()], 1, &[
        GTextEntry {
            text: String::new(),
            line_break: GTextLineBreak::End,
        },
    ]; "empty slice")]
    #[test_case(&["".to_string()], 1, &[
        GTextEntry {
            text: String::new(),
            line_break: GTextLineBreak::End,
        }
    ]; "one empty string")]
    #[test_case(&["test".to_string()], 1, &[
        GTextEntry {
            text: "test".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "one non-empty string")]
    #[test_case(&["test".to_string(), "".to_string()], 1, &[
        GTextEntry {
            text: "test".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "empty string at the end")]
    #[test_case(&["".to_string(), "test".to_string()], 1, &[
        GTextEntry {
            text: "test".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "empty string at the start")]
    #[test_case(&["test".to_string(), "".to_string(), "test2".to_string()], 2, &[
        GTextEntry {
            text: "test".to_string(),
            line_break: GTextLineBreak::NewParagraph,
        },
        GTextEntry {
            text: "test2".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "two lines with empty line between")]
    #[test_case(&[
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
    ], 3, &[
        GTextEntry {
            text: "test1".to_string(),
            line_break: GTextLineBreak::NewLine,
        },
        GTextEntry {
            text: "test2".to_string(),
            line_break: GTextLineBreak::NewLineWithScroll,
        },
        GTextEntry {
            text: "test3".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "three non-empty lines")]
    #[test_case(&[
        "test1".to_string(),
        "".to_string(),
        "test2".to_string(),
        "".to_string(),
        "test3".to_string(),
    ], 3, &[
        GTextEntry {
            text: "test1".to_string(),
            line_break: GTextLineBreak::NewParagraph,
        },
        GTextEntry {
            text: "test2".to_string(),
            line_break: GTextLineBreak::NewParagraph,
        },
        GTextEntry {
            text: "test3".to_string(),
            line_break: GTextLineBreak::End,
        },
    ]; "three non-empty lines with empty lines between")]
    fn test_gtext_block_from_plain_text(
        lines: &[String],
        expected_count: usize,
        expected_entries: &[GTextEntry],
    ) {
        let block = GTextBlock::from_plain_text(lines, "test_block");

        assert_eq!(block.entries.len(), expected_count);
        assert_eq!(block.entries, expected_entries);
    }
}
