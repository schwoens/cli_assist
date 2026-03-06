use anyhow::{Context, Result};
use clap::Parser;
use std::env::{self, split_paths};
use std::fs;
use std::str::FromStr;

use crate::command_tokenizer::TokenizedCmd;
use crate::levenshtein::get_levenshtein_distance;
use crate::man_page_parser::parse_command_options;

pub mod command_tokenizer;
pub mod line_tokenizer;
pub mod man_page_parser;
pub mod levenshtein;

fn main() {
    // println!("{}", get_current_shell());
    //
    // let shell = env::var("SHELL").expect("SHELL variable is not set");
    // println!("{}", shell);
    //
    // let previous_command = Command::new(shell)
    //     .arg("-c")
    //     .arg("history --max 2")
    //     .output()
    //     .expect("failed to fetch history");
    //
    // let previous_output = String::from_utf8(previous_command.stdout).expect("output is not a string");
    //
    // println!("{}", previous_output);

    let args = Args::parse();

    match correct_command(&args.input) {
        Ok(c) => println!("Did you mean {}?", c),
        Err(e) => eprintln!("Error correcting command: {}", e),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn correct_command(input: &str) -> Result<String> {

    let commands = input.split(";|><");

    let tokenized_cmd = TokenizedCmd::from_str(input)?;

    let command = tokenized_cmd.command();

    let closest_match = get_closest_match(&command)?;
    let options = parse_command_options(&closest_match);

    Ok(closest_match)
}

fn get_commands() -> Result<Vec<String>> {
    let path = env::var("PATH")?;
    let paths = split_paths(&path);
    let entries: Vec<String> = paths
        .flat_map(|sp| {
            fs::read_dir(sp)
                .unwrap()
                .map(|e| e.unwrap().file_name().into_string().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    Ok(entries)
}

fn get_closest_match(input: &str) -> Result<String> {
    let commands = get_commands()?;

    let mut results = Vec::new();

    for command in commands {
        let distance = get_levenshtein_distance(input, &command);
        results.push((command, distance));
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));

    let closest_match = results.first().context("no results")?;

    println!("distance: {}", closest_match.1);

    Ok(closest_match.0.clone())
}
