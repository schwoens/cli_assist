use std::collections::HashSet;

use anyhow::Result;
use regex::Regex;

pub fn parse_command_all_options(command: &str) -> Result<Vec<String>> {
    let man_page = get_man_page(command)?;
    let regex = Regex::new(r"(-([a-zA-Z]))|(--([a-z]|-)+)")?;
    let mut matches = vec![];

    for m in regex.captures_iter(&man_page) {
        matches.push(m[0].to_string()); 
    }

    Ok(matches)
}

pub fn parse_command_long_options(command: &str) -> Result<HashSet<String>> {
    let man_page = get_man_page(command)?;
    let regex = Regex::new(r"--([a-z]|-)+")?;
    let mut matches = HashSet::new();

    for m in regex.captures_iter(&man_page) {
        matches.insert(m[0].to_string());
    }

    Ok(matches)
}

fn get_man_page(command: &str) -> Result<String> {
    let man_output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("man {}", command))
        .output()?;

    Ok(String::from_utf8(man_output.stdout)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ls_long_options() {
        let result = parse_command_long_options("ls").expect("failed parsing options");
        let expected = HashSet::from([
            String::from("--all"),
            String::from("--almost-all"),
            String::from("--author"),
            String::from("--block-size"),
            String::from("--classify"),
            String::from("--color"),
            String::from("--context"),
            String::from("--dereference"),
            String::from("--dereference-command-line"),
            String::from("--dereference-command-line-symlink-to-dir"),
            String::from("--directory"),
            String::from("--dired"),
            String::from("--escape"),
            String::from("--file-type"),
            String::from("--format"),
            String::from("--full-time"),
            String::from("--group-directories-first"),
            String::from("--help"),
            String::from("--hide"),
            String::from("--hide-control-chars"),
            String::from("--human-readable"),
            String::from("--hyperlink"),
            String::from("--ignore"),
            String::from("--ignore-backups"),
            String::from("--indicator-style"),
            String::from("--inode"),
            String::from("--kibibytes"),
            String::from("--literal"),
            String::from("--no-group"),
            String::from("--numeric-uid-gid"),
            String::from("--quote-name"),
            String::from("--quoting-style"),
            String::from("--recursive"),
            String::from("--reverse"),
            String::from("--show-control-chars"),
            String::from("--si"),
            String::from("--size"),
            String::from("--sort"),
            String::from("--tabsize"),
            String::from("--time"),
            String::from("--time-style"),
            String::from("--version"),
            String::from("--width"),
            String::from("--zero"),
        ]);
        assert_eq!(result, expected);
    }
}
