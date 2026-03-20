use anyhow::{Context, Result, bail};
use std::{
    env::{self, split_paths},
    fs,
    path::PathBuf,
    process::Command,
};

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
                .map(|e| {
                    e.unwrap()
                        .file_name()
                        .into_string()
                        .expect("Invalid unicode in file name")
                })
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

/// Validates if a given shell is valid by checking the /etc/shells file.
/// It finds the path of a shell by using the `which` command.
/// Fails if the shells file is not readable or if the which command doesn't exist.
pub fn shell_is_valid(shell: &str) -> Result<bool> {
    const SHELLS_FILE_PATH: &str = "/etc/shells";
    let shell_path = get_command_path(shell)?;

    match shell_path {
        Some(sp) => Ok(fs::read_to_string(SHELLS_FILE_PATH)?
            .lines()
            .any(|l| l == &sp)),
        None => Ok(false),
    }
}

/// Returns the Path of a given command by calling the `which` command.
/// Fails if the which command doesn't exist.
pub fn get_command_path(command: &str) -> Result<Option<PathBuf>> {
    let path = String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(format!("which {}", command))
            .output()?
            .stdout,
    )?;

    if !path.is_empty() {
        Ok(Some(path.trim().into()))
    } else {
        Ok(None)
    }
}

/// Returns all environment variable keys.
pub fn get_environment_variables() -> Vec<String> {
    env::vars().map(|v| v.0).collect()
}
