use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum GTextLineBreak {
    NewLine,
    NewLineWithScroll,
    NewParagraph,
    End,
}

impl Display for GTextLineBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTextLineBreak::NewLine => write!(f, r"\n"),
            GTextLineBreak::NewLineWithScroll => write!(f, r"\l"),
            GTextLineBreak::NewParagraph => write!(f, r"\p"),
            GTextLineBreak::End => write!(f, "$"),
        }
    }
}

#[derive(Debug)]
pub struct GTextEntry {
    pub text: String,
    pub line_break: GTextLineBreak,
}

impl ToString for GTextEntry {
    fn to_string(&self) -> String {
        format!("\t.string \"{}{}\"", self.text, self.line_break)
    }
}

#[derive(Debug)]
pub struct GTextBlock {
    pub name: String,
    pub entries: Vec<GTextEntry>,
}

impl ToString for GTextBlock {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("gText_{}::\n", self.name));
        for entry in &self.entries {
            output.push_str(&entry.to_string());
            output.push('\n');
        }

        output
    }
}

impl GTextBlock {
    pub fn from_plain_text(lines: &[String], name: &str) -> Self {
        let mut entries = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }

            let linebreak;

            if (i == 0 || lines[i - 1].is_empty())
                && (i < lines.len() - 1 && lines[i + 1].is_empty())
            {
                linebreak = GTextLineBreak::NewParagraph;
            } else if i == 0 || lines[i - 1].is_empty() {
                linebreak = GTextLineBreak::NewLine;
            } else if i < lines.len() - 1 && !lines[i + 1].is_empty() {
                linebreak = GTextLineBreak::NewLineWithScroll;
            } else if i == lines.len() - 1 {
                linebreak = GTextLineBreak::End;
            } else {
                linebreak = GTextLineBreak::NewParagraph;
            }

            entries.push(GTextEntry {
                text: line.clone(),
                line_break: linebreak,
            });
        }

        Self {
            name: name.to_string(),
            entries,
        }
    }
}
