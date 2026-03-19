use std::collections::HashSet;

use anyhow::Result;
use regex::Regex;

/// Returns the available long options for a given command
pub fn parse_command_long_options(command: &str, shell: &str) -> Result<Vec<String>> {
    let mut page = get_man_page(command, shell)?;
    let regex = Regex::new(r"--([a-z]|-)+")?;
    let mut matches: HashSet<String> = HashSet::new();

    if page.is_none() {
        page = get_help_page(command, shell)?;
    }

    for m in regex.captures_iter(&page.unwrap()) {
        matches.insert(m[0].to_string());
    }

    Ok(matches.iter().cloned().collect())
}

/// Executes the `man` command for a given command and returns its output. 
/// Returns None if no man page exists for the given command.
pub fn get_man_page(command: &str, shell: &str) -> Result<Option<String>> {
    let man_output = std::process::Command::new(shell)
        .arg("-c")
        .arg(format!("man {}", command))
        .output()?;

    let man_page = String::from_utf8(man_output.stdout)?;

    if man_page.is_empty() {
        return Ok(None);
    }

    Ok(Some(man_page))
}

/// Executes a given command with the `--help` option and returns its output.
/// Returns None if no help page exists.
pub fn get_help_page(command: &str, shell: &str) -> Result<Option<String>> {
    let help_output = std::process::Command::new(shell)
        .arg("-c")
        .arg(format!("{} --help", command))
        .output()?;

    let help_page = String::from_utf8(help_output.stdout)?;

    if help_page.is_empty() {
        return Ok(None);
    }

    Ok(Some(help_page))
}
