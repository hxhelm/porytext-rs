use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Font {
    max_line_length: usize,
    widths: HashMap<String, usize>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FontConfig {
    pub default_font_id: String,
    pub fonts: HashMap<String, Font>,
}

impl FontConfig {
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let font_config = std::fs::read_to_string(path)?;
        serde_json::from_str(&font_config).map_err(std::convert::Into::into)
    }

    pub fn get_font(&self, font_id: Option<&str>) -> Result<&Font, Box<dyn Error>> {
        let font_id = font_id.unwrap_or(self.default_font_id.as_str());

        self.fonts
            .get(font_id)
            .ok_or_else(|| format!("Font with id '{font_id}' not found in font config file").into())
    }
}

// TODO: support special words contained in curly brackets like {PLAYER}
pub fn split_line(input: &str, font: &Font) -> Vec<String> {
    let mut output = vec![String::new()];
    let mut line_width = 0;
    let space_width = font.widths.get(" ").unwrap();

    for word in input.split_whitespace() {
        let mut tmp_utf8_char = [0u8; 4];
        let word_width = word.chars().fold(0, |acc, c| {
            acc + font.widths.get(c.encode_utf8(&mut tmp_utf8_char)).unwrap()
        });

        let line = output.last_mut().unwrap();

        if line_width + word_width >= font.max_line_length {
            output.push(word.to_string());
            line_width = word_width;
            continue;
        }

        if !line.is_empty() {
            line.push(' ');
        }

        line.push_str(word);
        line_width += space_width + word_width;
    }

    output
}
