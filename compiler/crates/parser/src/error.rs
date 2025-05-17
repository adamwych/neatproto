use neatproto_ast::{LocalizedToken, Token};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnknownIdentifier,
    ExpectedIdentifier,
    ExpectedTokenOfKind(Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::UnknownIdentifier => write!(f, "Unknown identifier"),
            ParseError::ExpectedIdentifier => write!(f, "Expected an identifier"),
            ParseError::ExpectedTokenOfKind(kind) => write!(f, "Expected '{}'", kind.value()),
        }
    }
}

#[derive(Debug)]
pub struct LocalizedParseError {
    pub error: ParseError,
    pub token: LocalizedToken,
}

impl Display for LocalizedParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if matches!(
            self.error,
            ParseError::UnexpectedToken | ParseError::UnknownIdentifier
        ) {
            write!(
                f,
                "{} '{}' in file '{}' at line {}:{}",
                self.error,
                self.token.value(),
                self.token.location.file_path,
                self.token.location.line,
                self.token.location.column
            )
        } else {
            write!(
                f,
                "{} in file '{}' at line {}:{}",
                self.error,
                self.token.location.file_path,
                self.token.location.line,
                self.token.location.column
            )
        }
    }
}
