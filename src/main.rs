use clap::Parser;
use gtext::GTextBlock;
use std::fs;
use std::path::PathBuf;
use text::FontConfig;

mod gtext;
mod text;

const DEFAULT_FONT_CONFIG: &str = "font_config.json";

/// A simple CLI tool to convert plain text to gText entries
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file to read from
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Set the id for the font to use, default is defined in the font config file
    #[arg(short = 'f', long)]
    font_id: Option<String>,

    /// Set the path to the font config file, default is ./font_config.json
    #[arg(short = 'c', long, value_name = "FILE")]
    font_config: Option<PathBuf>,

    /// The name of the generated gText block
    #[arg(short, long)]
    block_name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_path = args.input;

    let input = match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(error) => {
            return println!(
                "Error reading input file '{}': {}",
                input_path.display(),
                error
            )
        }
    };

    let font_config_path = args
        .font_config
        .unwrap_or_else(|| PathBuf::from(DEFAULT_FONT_CONFIG));

    let font_config = match FontConfig::from_file(&font_config_path) {
        Ok(font_config) => font_config,
        Err(error) => {
            return println!(
                "Error parsing font_config file '{}': {}",
                font_config_path.display(),
                error
            )
        }
    };

    let font = match font_config.get_font(args.font_id.as_deref()) {
        Ok(font_config) => font_config,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    let block_name = args.block_name.unwrap_or_else(|| String::from("<NAME>"));

    let lines = input
        .lines()
        .map(String::from)
        .flat_map(|line| text::split_line(&line, font))
        .collect::<Vec<String>>();

    let g_text = GTextBlock::from_plain_text(&lines, &block_name);

    println!("{}", g_text.to_string());
}
