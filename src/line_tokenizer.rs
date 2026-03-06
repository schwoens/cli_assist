use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedLine {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
struct Token(TokenType, String);

#[derive(Debug, PartialEq, Eq, Clone)]
enum TokenType {
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

impl FromStr for TokenizedLine {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut current_token_type = TokenType::Command;
        let mut escape_character = None;

        for char in str.chars() {
            match char {
                '\'' | '\"' | '\\' => {
                    match escape_character {
                        Some('\\') if char == '\\' => { 
                            current_token.push(char);
                            escape_character = None;
                        },
                        Some(_) if escape_character.is_some_and(|ec| ec == char) => escape_character = None,
                        None => escape_character = Some(char),
                        _ => {
                            if let TokenType::Space = current_token_type {
                                current_token_type = TokenType::Argument;
                            }
                            current_token.push(char);
                        }
                    }
                    
                }
                ' ' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Command
                            | TokenType::Argument
                            | TokenType::Literal
                            | TokenType::ShortOption
                            | TokenType::LongOption => {
                                tokens.push(Token(current_token_type.clone(), current_token.clone()));
                                current_token.clear();
                                current_token_type = TokenType::Space;
                            },
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
                            TokenType::Space | TokenType::Redirect | TokenType::NumericArgument => (),
                            _ => {
                                tokens.push(Token(current_token_type.clone(), current_token.clone()));
                            }
                        }
                        current_token.clear();
                        current_token_type = TokenType::Redirect;
                    } else {
                        current_token.push(char)
                    }
                },
                '|' => {
                    if escape_character.is_none() {
                        match current_token_type {
                            TokenType::Space | TokenType::Redirect => (),
                            _ => {
                                tokens.push(Token(current_token_type.clone(), current_token.clone()));
                                current_token.clear();
                            }
                        }
                        current_token_type = TokenType::Pipe;
                    } else {
                        current_token.push(char);
                    }
                },
                '0'..='9' => {
                    if current_token_type == TokenType::Space { current_token_type = TokenType::NumericArgument }
                    current_token.push(char);
                },
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_command_and_long_option() {
        let result = TokenizedLine::from_str("ls --all").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("ls")),
                Token(TokenType::LongOption, String::from("--all")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_and_short_option() {
        let result = TokenizedLine::from_str("ls -C").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("ls")),
                Token(TokenType::ShortOption, String::from("-C")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_pipe_no_spaces() {
        let result = TokenizedLine::from_str("echo|cat").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Command, String::from("cat")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_argument_and_pipe() {
        let result = TokenizedLine::from_str("echo hello | cat").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello")),
                Token(TokenType::Command, String::from("cat")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_argument_and_output_redirect() {
        let result = TokenizedLine::from_str("echo hello > file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_output_redirect_no_spaces() {
        let result = TokenizedLine::from_str("ls>file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("ls")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_argument_and_append_output_redirect() {
        let result = TokenizedLine::from_str("echo hello >> file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_argument_and_input_redirect() {
        let result = TokenizedLine::from_str("echo hello < file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_argument_and_numbered_output_redirect() {
        let result = TokenizedLine::from_str("echo hello 2> file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_numbered_output_redirect_no_spaces() {
        let result = TokenizedLine::from_str("echo hello2>file").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("hello2")),
                Token(TokenType::Literal, String::from("file")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_with_variable_argument() {
        let result = TokenizedLine::from_str("echo $SHELL").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("$SHELL")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_with_single_quote_argument() {
        let result = TokenizedLine::from_str("echo 'test|><;\\\"'").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("test|><;\\\"")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_with_double_quote_argument() {
        let result = TokenizedLine::from_str("echo \"test|><;\\\'\"").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("test|><;\\\'")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_backslash_escape_space() {
        let result = TokenizedLine::from_str("echo \\ ").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from(" ")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_backslash_escape_single_quote() {
        let result = TokenizedLine::from_str("echo \\'").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("'")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_backslash_escape_double_quote() {
        let result = TokenizedLine::from_str("echo \\\"").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("echo")),
                Token(TokenType::Argument, String::from("\"")),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_command_single_quote_escape_after_redirect_no_spaces() {
        let result = TokenizedLine::from_str("ls>'test\"'").expect("failed to tokenize");
        let expected = TokenizedLine {
            tokens: vec![
                Token(TokenType::Command, String::from("ls")),
                Token(TokenType::Literal, String::from("test\"")),
            ],
        };
        assert_eq!(result, expected);
    }
}
