use anyhow::{Context, Result, bail};
use std::{env::{self, split_paths}, fs, process::Command};

/// Returns the previously executed command by querying the shell history.
pub fn get_previous_command(shell: &str) -> Result<String> {
    let history = String::from_utf8(
        Command::new(shell)
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

/// Returns a Vec of all available commands on the system by listing every file in every PATH
/// directory. Fails if the PATH environment variable is not or invalid.
pub fn get_available_commands() -> Result<Vec<String>> {
    let path = env::var("PATH")?;
    let paths = split_paths(&path);
    let entries: Vec<String> = paths
        .flat_map(|sp| {
            fs::read_dir(sp)
                .expect("Invalid PATH variable")
                .map(|e| e.unwrap().file_name().into_string().expect("Invalid unicode in file name"))
                .collect::<Vec<_>>()
        })
        .collect();
    Ok(entries)
}

/// Returns the content of the SHELL environment variable.
/// Fails if the environment variable is not set or empty.
pub fn get_shell_from_env_variable() -> Result<String> {
    let shell_var = env::var("SHELL").context("SHELL environment variable is not set")?;
    if shell_var.is_empty() {
        bail!("SHELL enviroment variable is empty");
    }
    Ok(shell_var)
}

