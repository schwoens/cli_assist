use anyhow::{Context, Result};
use clap::Parser;
use std::env::{self, split_paths};
use std::fs;

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

#[derive(Debug, PartialEq, Eq)]
struct TokenizedCmd {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
struct Token(TokenType, String);

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Operator,
    Literal,
    SingleQuoteLiteral,
    DoubleQuoteLiteral,
    Delimiter,
    ShortOption,
    LongOption,
}

impl TokenizedCmd {
    fn from_str(str: &str) -> Self {
        println!("str: '{}'", str);
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut current_token_type = None;
        for char in str.chars() {
            match char {
                '-' => {
                    match current_token_type {
                        Some(TokenType::ShortOption) => current_token_type = Some(TokenType::LongOption),
                        None => current_token_type = Some(TokenType::ShortOption),
                        _ => (),
                    }
                },
                ' ' | ';' | '|' | '<' | '>' => {
                    if current_token_type.is_some() {
                        tokens.push(Token(current_token_type.expect("missing token type"), current_token.clone()));
                        current_token_type = None;
                        current_token.clear();
                    }
                    tokens.push(Token(TokenType::Delimiter, char.to_string()));
                    continue;
                },
                '\'' => {
                    if current_token_type.is_none() {
                        current_token_type = Some(TokenType::SingleQuoteLiteral)
                    }
                },
                '\"' => {
                    if current_token_type.is_none() {
                        current_token_type = Some(TokenType::DoubleQuoteLiteral)
                    }
                }
                _ => {
                    if current_token_type.is_none() {
                        current_token_type = Some(TokenType::Literal);
                    }
                }
            }
            current_token.push(char);
        }
        tokens.push(Token(current_token_type.expect("missing token type"), current_token));
        Self { tokens }
    }
}

fn correct_command(input: &str) -> Result<String> {

    let tokenized_cmd = TokenizedCmd::from_str(input);

    println!("{:?}", tokenized_cmd);

    todo!();

    let closest_match = get_closest_match(input)?;
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

fn parse_command_options(command: &str) -> Result<Vec<String>> {
    let man_output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("man {}", command))
        .output()?;
    let man_page = String::from_utf8(man_output.stdout)?;

    println!("{}", man_page);
    todo!();
}

fn get_levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    if s1_len == 0 {
        return s2_len;
    }

    if s2_len == 0 {
        return s1_len;
    }

    let mut cache: Vec<usize> = (1..).take(s1_len).collect();
    let mut result = 0;

    for (s2_index, s2_code) in s2.chars().enumerate() {
        result = s2_index;
        let mut s1_distance = s2_index;

        for (s1_index, s1_code) in s1.chars().enumerate() {
            let s2_distance = if s1_code == s2_code {
                s1_distance
            } else {
                s1_distance + 1
            };

            s1_distance = cache[s1_index];

            result = if s1_distance > result {
                if s2_distance > result {
                    result + 1
                } else {
                    s2_distance
                }
            } else if s2_distance > s1_distance {
                s1_distance + 1
            } else {
                s2_distance
            };

            cache[s1_index] = result;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn levenshtein_distance_1() {
        let result = get_levenshtein_distance("kitten", "sitting");
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn levenshtein_distance_2() {
        let result = get_levenshtein_distance("uninformed", "uniformed");
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_1() {
        let result = TokenizedCmd::from_str("ls --all /usr/bin -C");
        let expected = TokenizedCmd { tokens: vec![
            Token(TokenType::Literal, String::from("ls")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::LongOption, String::from("--all")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::Literal, String::from("/usr/bin")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::ShortOption, String::from("-C"))
        ]};
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_2() {
        let result = TokenizedCmd::from_str("echo hello | cat");
        let expected = TokenizedCmd { tokens: vec![
            Token(TokenType::Literal, String::from("echo")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::Literal, String::from("hello")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::Delimiter, String::from("|")),
            Token(TokenType::Delimiter, String::from(" ")),
            Token(TokenType::Literal, String::from("cat"))
        ]};
        assert_eq!(result, expected);
    }
}
