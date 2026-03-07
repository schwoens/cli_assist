use anyhow::{Context, Result};
use clap::Parser;
use std::env::{self, split_paths};
use std::fs;
use std::str::FromStr;

use crate::levenshtein::get_levenshtein_distance;
use crate::line_tokenizer::TokenizedLine;
use crate::man_page_parser::parse_command_long_options;

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

    let tokenized_line = TokenizedLine::from_str(input);
    let commands = tokenized_line?.get_commands_with_options()?;
    let mut output = input.to_string();

    for command in commands {
        let closest_command_match = get_closest_command_match(&command.name)?;

        output = output.replace(&command.name, &closest_command_match);

        let long_options = command.get_long_options();

        for option in long_options {
            let closest_option_match = get_closest_long_option_match(&closest_command_match, &option)?;

            output = output.replace(&option, &closest_option_match);
        }
    }
    Ok(output)
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

fn get_closest_command_match(input: &str) -> Result<String> {
    let commands = get_commands()?;

    let mut results = Vec::new();

    for command in commands {
        let distance = get_levenshtein_distance(input, &command);
        results.push((command, distance));
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));

    let closest_match = results.first().context("no command matches")?;

    Ok(closest_match.0.clone())
}

fn get_closest_long_option_match(command: &str, option: &str) -> Result<String> {
    let long_options = parse_command_long_options(command)?;

    println!("{}", command);
    println!("{:?}", long_options);

    let mut results = Vec::new();

    for opt in long_options {
        let distance = get_levenshtein_distance(option, &opt);
        results.push((opt, distance));
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));

    let closest_match = results.first().context("no option matches")?;

    Ok(closest_match.0.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_command_name_only() {
        let result = correct_command("chmad").expect("unable to correct command");
        let expected = "chmod";
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_long_option_only() {
        let result = correct_command("ls --allmost-all").expect("unable to correct command");
        let expected = "ls --almost-all";
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_command_name_and_long_option() {
        let result = correct_command("mcdir --partens").expect("unable to correct command");
        let expected = "mkdir --parents";
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_multiple_commands() {
        let result = correct_command("ls --allmost-all | mcdir --partens").expect("unable to correct command");
        let expected = "ls --almost-all | mkdir --parents";
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_correct_command_with_correct_long_options() {
        let result = correct_command("touch --no-create").expect("unable to correct command");
        let expected = "touch --no-create";
        assert_eq!(result, expected);
    }
}
