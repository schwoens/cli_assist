use std::str::FromStr;
use crate::tokenize::{TokenizedCommand, Token, TokenType, TokenizedLine};

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
fn tokenize_command_argument_and_pipe_and_ampersand() {
    let result = TokenizedLine::from_str("echo hello |& cat").expect("failed to tokenize");
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
fn tokenize_command_argument_and_pipe_and_ampersand_reverse() {
    let result = TokenizedLine::from_str("echo hello &| cat").expect("failed to tokenize");
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
fn tokenize_command_argument_and_double_pipe() {
    let result = TokenizedLine::from_str("echo hello || cat").expect("failed to tokenize");
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
fn tokenize_command_argument_and_double_ampersand() {
    let result = TokenizedLine::from_str("echo hello && cat").expect("failed to tokenize");
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
fn tokenize_command_argument_and_semicolon() {
    let result = TokenizedLine::from_str("echo hello; echo world").expect("failed to tokenize");
    let expected = TokenizedLine {
        tokens: vec![
            Token(TokenType::Command, String::from("echo")),
            Token(TokenType::Argument, String::from("hello")),
            Token(TokenType::Command, String::from("echo")),
            Token(TokenType::Argument, String::from("world")),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn tokenize_command_argument_and_ampersand() {
    let result = TokenizedLine::from_str("top & echo hello").expect("failed to tokenize");
    let expected = TokenizedLine {
        tokens: vec![
            Token(TokenType::Command, String::from("top")),
            Token(TokenType::Command, String::from("echo")),
            Token(TokenType::Argument, String::from("hello")),
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

#[test]
fn get_commands_with_options() {
    let tokenized_line = TokenizedLine {
        tokens: vec![
            Token(TokenType::Command, String::from("ls")),
            Token(TokenType::Literal, String::from("test")),
            Token(TokenType::LongOption, String::from("--all")),
            Token(TokenType::ShortOption, String::from("-C")),
        ],
    };
    let result = tokenized_line.get_commands_with_options().expect("failed to get commands with options");
    let expected = vec![TokenizedCommand {
        name: String::from("ls"),
        options: vec![
            Token(TokenType::LongOption, String::from("--all")),
            Token(TokenType::ShortOption, String::from("-C")),
        ],
    }];
    assert_eq!(result, expected);
}
