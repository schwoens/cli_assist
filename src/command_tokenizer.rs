use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedCmd {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
struct Token(TokenType, String);

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Literal,
    SingleQuoteLiteral,
    DoubleQuoteLiteral,
    Delimiter,
    ShortOption,
    LongOption,
}

impl TokenizedCmd {
    pub fn command(&self) -> String {
        self.tokens.first().expect("command shouldn't be empty").1.clone()
    }
}

impl FromStr for TokenizedCmd {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
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
        Ok(Self { tokens })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_1() {
        let result = TokenizedCmd::from_str("ls --all /usr/bin -C").expect("failed to tokenize");
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
        let result = TokenizedCmd::from_str("echo hello | cat").expect("failed to tokenize");
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
