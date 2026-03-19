use anyhow::Result;
use clap::Parser;
use std::{io::{self, Write}, process::Command};
use std::str::FromStr;

use crate::{doc_parse::parse_command_long_options, shell_query::get_shell_from_env_variable};
use crate::levenshtein::get_closest_match;
use crate::shell_query::{get_available_commands, get_previous_command};
use crate::tokenize::TokenizedLine;

pub mod doc_parse;
pub mod levenshtein;
pub mod shell_query;
pub mod test;
pub mod tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    command: Option<String>,

    #[arg(short, long)]
    shell: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(&args)
}

fn run(args: &Args) -> Result<()> {
    let shell = match &args.shell {
        Some(s) => s,
        None => &get_shell_from_env_variable()?,
    };

    let command = match &args.command {
        Some(c) => c,
        None => &get_previous_command(shell)?,
    };

    match correct_command(command, shell)? {
        Some(c) => {
            if user_confirms(&c)? {
                run_command(&c, &args.shell.clone().unwrap_or(String::from("sh")))?
            }
        }
        _ => {
            println!("No correction available");
        },
    }
    Ok(())
}

/// Executes the given command in a given shell.
fn run_command(command: &str, shell: &str) -> Result<()> {
    let output = Command::new(shell).arg("-c").arg(command).output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

/// Tries to correct the given line.
/// Returns Some if the correction was successful, otherwise it will return None.
fn correct_command(input: &str, shell: &str) -> Result<Option<String>> {
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
        let available_long_options = parse_command_long_options(
            &closest_command_match.clone().unwrap_or(input.to_string()),
            shell,
        )?;

        for option in long_options {
            let closest_option_match = get_closest_match(&option, &available_long_options)?;

            output = output.replace(&option, &closest_option_match.unwrap_or(option.clone()));
        }
    }

    if output == input {
        return Ok(None);
    }

    Ok(Some(output))
}

/// Queries the user to confirm the given correction
fn user_confirms(correction: &str) -> Result<bool> {
    println!("{} [Y/n]", correction);

    let stdin = std::io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input)?;

    match input.to_lowercase().as_str() {
        "\n" | "y\n" => Ok(true),
        _ => Ok(false),
    }
}
