use std::str::FromStr;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedLine {
    pub(crate) tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token(pub TokenType, pub String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Argument,
    Literal,
    Pipe,
    Command,
    Space,
    ShortOption,
    LongOption,
    Redirect,
    NumericArgument,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandWithOptions {
    pub name: String,
    pub options: Vec<Token>,
}

impl CommandWithOptions {
    pub fn get_long_options(&self) -> Vec<String> {
        self.options
            .iter()
            .filter(|o| o.0 == TokenType::LongOption)
            .map(|o| o.1.clone())
            .collect()
    }
}

impl TokenizedLine {
    pub fn get_commands_with_options(&self) -> Result<Vec<CommandWithOptions>> {

        let mut commands = Vec::new();
        let mut current_command_name = None;
        let mut current_options = Vec::new();

        for token in &self.tokens {
            match token.0 {
                TokenType::Command if current_command_name.is_none() => {
                    current_command_name = Some(token.1.clone())
                }
                TokenType::Command if current_command_name.is_some() => {
                    commands.push(CommandWithOptions {
                        name: current_command_name.unwrap(),
                        options: current_options.clone(),
                    });
                    current_command_name = Some(token.1.clone());
                    current_options.clear();
                }
                TokenType::LongOption if current_command_name.is_some() => current_options.push(token.clone()),
                TokenType::ShortOption if current_command_name.is_some() => current_options.push(token.clone()),
                _ => (),
            }
        }
        commands.push(CommandWithOptions {
            name: current_command_name.context("no command name")?,
            options: current_options,
        });
        Ok(commands)
    }
}

impl FromStr for TokenizedLine {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut current_token_type = TokenType::Command;
        let mut escape_character = None;

        for char in str.chars() {
            match char {
                '\'' | '\"' | '\\' => match escape_character {
                    Some('\\') if char == '\\' => {
                        current_token.push(char);
                        escape_character = None;
                    }
                    Some(_) if escape_character.is_some_and(|ec| ec == char) => {
                        escape_character = None
                    }
                    None => escape_character = Some(char),
                    _ => {
                        if let TokenType::Space = current_token_type {
                            current_token_type = TokenType::Argument;
                        }
                        current_token.push(char);
                    }
                },
                ' ' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Command
                            | TokenType::Argument
                            | TokenType::Literal
                            | TokenType::ShortOption
                            | TokenType::LongOption => {
                                tokens
                                    .push(Token(current_token_type.clone(), current_token.clone()));
                                current_token.clear();
                                current_token_type = TokenType::Space;
                            }
                            TokenType::Pipe | TokenType::Redirect => (),
                            _ => current_token_type = TokenType::Space,
                        }
                    } else {
                        current_token_type = TokenType::Argument;
                        current_token.push(char);
                    }
                }
                '-' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Space => current_token_type = TokenType::ShortOption,
                            TokenType::ShortOption => current_token_type = TokenType::LongOption,
                            _ => (),
                        }
                    }
                    current_token.push(char);
                }
                '>' | '<' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Space | TokenType::Redirect | TokenType::NumericArgument => {}
                            _ => {
                                tokens
                                    .push(Token(current_token_type.clone(), current_token.clone()));
                            }
                        }
                        current_token.clear();
                        current_token_type = TokenType::Redirect;
                    } else {
                        current_token.push(char)
                    }
                }
                '|' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Space | TokenType::Redirect => (),
                            _ => {
                                tokens
                                    .push(Token(current_token_type.clone(), current_token.clone()));
                                current_token.clear();
                            }
                        }
                        current_token_type = TokenType::Pipe;
                    } else {
                        current_token.push(char);
                    }
                }
                '0'..='9' => {
                    if current_token_type == TokenType::Space {
                        current_token_type = TokenType::NumericArgument
                    }
                    current_token.push(char);
                }
                _ => {
                    match current_token_type {
                        TokenType::Space => current_token_type = TokenType::Argument,
                        TokenType::Pipe => current_token_type = TokenType::Command,
                        TokenType::Redirect => current_token_type = TokenType::Literal,
                        _ => (),
                    }
                    current_token.push(char);
                }
            }
        }
        tokens.push(Token(current_token_type, current_token));
        Ok(Self { tokens })
    }
}
