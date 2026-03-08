use anyhow::{Context, Result, bail};
use clap::Parser;
use std::env::{self, split_paths};
use std::fs;
use std::process::{Command, exit};
use std::str::FromStr;

use crate::levenshtein::{get_closest_match, get_levenshtein_distance};
use crate::line_tokenizer::TokenizedLine;
use crate::man_page_parser::parse_command_long_options;

pub mod levenshtein;
pub mod line_tokenizer;
pub mod man_page_parser;

fn main() {
    let args = Args::parse();

    let command = match args.command {
        Some(c) => c,
        None => match get_previous_command(args.shell) {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                exit(0);
            }
        },
    };

    match correct_command(&command) {
        Ok(Some(c)) => println!("{}", c),
        Ok(None) => println!("No correction available"),
        Err(e) => eprintln!("Error correcting command: {}", e),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    command: Option<String>,

    #[arg(short, long)]
    shell: Option<String>,
}

fn get_previous_command(override_shell: Option<String>) -> Result<String> {
    let shell = match override_shell {
        Some(s) => s,
        None => {
            let shell_var = env::var("SHELL").context("SHELL enviroment variable is not set")?;
            if shell_var.is_empty() {
                bail!("SHELL enviroment variable is empty");
            }
            shell_var
        }
    };

    let history = String::from_utf8(
        Command::new(&shell)
            .arg("-c")
            .arg("history")
            .output()
            .context(format!("{} is not a supported shell", shell))?
            .stdout,
    )?;

    Ok(history
        .lines()
        .take(2)
        .last()
        .context("shell history is empty")?
        .to_string())
}

fn correct_command(input: &str) -> Result<Option<String>> {
    let tokenized_line = TokenizedLine::from_str(input);
    let contained_commands = tokenized_line?.get_commands_with_options()?;
    let mut output = input.to_string();

    let available_commands = get_available_commands()?;

    for command in contained_commands {
        let closest_command_match = &get_closest_match(&command.name, &available_commands)?;

        if let Some(m) = closest_command_match {
            output = output.replace(&command.name, m)
        }

        let long_options = command.get_long_options();
        let available_long_options =
            parse_command_long_options(&closest_command_match.clone().unwrap_or(input.to_string()))?;

        for option in long_options {
            let closest_option_match = get_closest_match(
                &option,
                &available_long_options,
            )?;

            output = output.replace(&option, &closest_option_match.unwrap_or(option.clone()));
        }
    }

    if output == input {
        return Ok(None);
    }

    Ok(Some(output))
}

fn get_available_commands() -> Result<Vec<String>> {
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
    let commands = get_available_commands()?;

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
        let expected = Some(String::from("chmod"));
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_long_option_only() {
        let result = correct_command("ls --allmost-all").expect("unable to correct command");
        let expected = Some(String::from("ls --almost-all"));
        assert_eq!(result, expected);
    }

    // #[test]
    // fn correct_command_name_and_long_option() {
    //     let result = correct_command("mcdir --partens").expect("unable to correct command");
    //     let expected = Some(String::from("mkdir --parents"));
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn correct_multiple_commands() {
    //     let result = correct_command("ls --allmost-all | mcdir --partens")
    //         .expect("unable to correct command");
    //     let expected = Some(String::from("ls --almost-all | mkdir --parents"));
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn correct_correct_command_with_correct_long_options() {
        let result = correct_command("touch --no-create").expect("unable to correct command");
        let expected = None;
        assert_eq!(result, expected);
    }
}
