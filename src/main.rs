use clap::Parser;
use gtext::{GTextBlock, GTextEntry, GTextLineBreak};
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

mod gtext;

const MAX_LINE_LENGTH: usize = 38;

/// A simple CLI tool to convert plain text to gText entries
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file to read from
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// The name of the generated gText block
    #[arg(short, long)]
    block_name: Option<String>,
}

fn split_line(input: &str) -> Vec<String> {
    let mut output = vec![String::new()];

    for word in input.split_whitespace() {
        let line = output.last_mut().unwrap();

        if line.len() + word.len() + 1 > MAX_LINE_LENGTH {
            output.push(word.to_string());
            continue;
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
    }

    output
}

fn main() {
    let args = Args::parse();

    let input_path = args.input;

    let input = match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(error) => {
            println!(
                "Error reading input file '{}': {}",
                input_path.display(),
                error.to_string()
            );
            return;
        }
    };

    let block_name = args.block_name.unwrap_or(String::from("<NAME>"));

    let lines = input
        .lines()
        .map(String::from)
        .flat_map(|line| split_line(&line))
        .collect::<Vec<String>>();

    let g_text = GTextBlock::from_plain_text(&lines, &block_name);

    println!("{}", g_text.to_string());
}
