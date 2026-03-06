use anyhow::Result;
use regex::Regex;

pub fn parse_command_options(command: &str) -> Result<Vec<String>> {
    let man_page = get_man_page(command)?;
    let regex = Regex::new(r"(-([a-zA-Z]))|(--([a-z]|-)+)")?;

    let lines = man_page.lines().filter(|l| regex.is_match(l));

    println!("{:?}", lines);

    todo!();
}

fn get_man_page(command: &str) -> Result<String> {
    let man_output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("man {}", command))
        .output()?;

    Ok(String::from_utf8(man_output.stdout)?)
}


