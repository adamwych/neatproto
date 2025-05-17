use crate::source_file::SourceFile;
use crate::{LocalizedParseError, ParseError, ParseResult};
use neatproto_ast::{LocalizedToken, SourceLocation, Token};
use std::str::Chars;

pub struct Tokens<'a> {
    _source_file: &'a SourceFile,
    chars: Chars<'a>,
    pub location: SourceLocation,
}

impl<'a> Tokens<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        Self {
            _source_file: source_file,
            chars: source_file.contents.chars(),
            location: SourceLocation {
                file_path: source_file.path.clone(),
                column: 1,
                line: 1,
            },
        }
    }

    pub fn next_identifier(&mut self) -> ParseResult<LocalizedToken> {
        let token = self.next().expect("unexpected end of file");
        if matches!(token.token, Token::Identifier(_)) {
            return Ok(token);
        }
        Err(LocalizedParseError {
            error: ParseError::ExpectedIdentifier,
            token,
        })
    }

    pub fn next_kind(&mut self, kind: Token) -> ParseResult<LocalizedToken> {
        let token = self.next().expect("unexpected end of file");
        if token.token == kind {
            return Ok(token);
        }
        Err(LocalizedParseError {
            error: ParseError::ExpectedTokenOfKind(kind),
            token,
        })
    }

    fn read_identifier(&mut self, c: char) -> Option<Token> {
        if !c.is_alphabetic() && c != '_' {
            return None;
        }

        let mut value = String::with_capacity(16);
        value.push(c);

        let mut it = self.chars.clone();
        while let Some(c) = it.next() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            value.push(c);
            self.chars.next();
        }

        Some(Token::Identifier(value))
    }

    fn read_digit(&mut self, c: char) -> Option<Token> {
        if !c.is_digit(10) {
            return None;
        }

        let mut value = String::with_capacity(8);
        value.push(c);

        let mut it = self.chars.clone();
        while let Some(c) = it.next() {
            if !c.is_digit(10) {
                break;
            }

            value.push(c);
            self.chars.next();
        }

        Some(Token::Digit(value))
    }

    fn read_string(&mut self, c: char) -> Option<Token> {
        if c != '"' {
            return None;
        }

        let mut value = String::with_capacity(16);

        while let Some(c) = self.chars.next() {
            if c == '"' {
                break;
            }

            value.push(c);
        }

        Some(Token::String(value))
    }

    fn read_special_character(&mut self, c: char) -> Option<Token> {
        match c {
            '(' => Some(Token::ParenOpen),
            ')' => Some(Token::ParenClose),
            '{' => Some(Token::BraceOpen),
            '}' => Some(Token::BraceClose),
            ':' => Some(Token::Colon),
            ';' => Some(Token::Semicolon),
            '=' => Some(Token::Equal),
            '@' => Some(Token::At),
            ',' => Some(Token::Comma),
            _ => Some(Token::Unknown(c)),
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = LocalizedToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(c) => {
                if c == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                    return self.next();
                }

                if c.is_whitespace() {
                    self.location.column += 1;
                    return self.next();
                }

                match self
                    .read_identifier(c)
                    .or_else(|| self.read_digit(c))
                    .or_else(|| self.read_string(c))
                    .or_else(|| self.read_special_character(c))
                {
                    Some(token) => {
                        let start_location = self.location.clone();

                        self.location.column += token.len();

                        Some(LocalizedToken {
                            token,
                            location: start_location,
                        })
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}
