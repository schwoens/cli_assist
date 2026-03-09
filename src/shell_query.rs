use anyhow::{Context, Result, bail};
use std::{env::{self, split_paths}, fs, process::Command};

/// Returns the previously executed command by querying the shell history.
pub fn get_previous_command(override_shell: Option<String>) -> Result<String> {
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

/// Returns a Vec of all available commands on the system by listing every file in every PATH
/// directory.
pub fn get_available_commands() -> Result<Vec<String>> {
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

