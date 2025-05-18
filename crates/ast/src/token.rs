use crate::SourceLocation;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    Digit(String),
    String(String),

    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    SquareOpen,
    SquareClose,
    Colon,
    Semicolon,
    Equal,
    At,
    Comma,

    Unknown(char),
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::Identifier(value) | Token::Digit(value) | Token::String(value) => value.clone(),
            Token::ParenOpen => "(".to_string(),
            Token::ParenClose => ")".to_string(),
            Token::BraceOpen => "{".to_string(),
            Token::BraceClose => "}".to_string(),
            Token::SquareOpen => "[".to_string(),
            Token::SquareClose => "]".to_string(),
            Token::Colon => ":".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Equal => "=".to_string(),
            Token::At => "@".to_string(),
            Token::Comma => ",".to_string(),
            Token::Unknown(character) => character.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Token::Identifier(value) | Token::Digit(value) | Token::String(value) => value.len(),
            _ => 1,
        }
    }
}

#[derive(Debug)]
pub struct LocalizedToken {
    pub token: Token,
    pub location: SourceLocation,
}

impl LocalizedToken {
    pub fn value(&self) -> String {
        self.token.value()
    }

    pub fn len(&self) -> usize {
        self.token.len()
    }
}
