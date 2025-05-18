use neatproto_ast::{SourceLocation, Token};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnknownIdentifier(String),
    ExpectedIdentifier,
    ExpectedTokenOfKind(Token),
    UnexpectedEndOfFile,
    ExpectedLiteral,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token '{}'", token.value()),
            ParseError::UnknownIdentifier(str) => write!(f, "Unknown identifier '{}'", str),
            ParseError::ExpectedIdentifier => write!(f, "Expected an identifier"),
            ParseError::ExpectedTokenOfKind(kind) => write!(f, "Expected '{}'", kind.value()),
            ParseError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            ParseError::ExpectedLiteral => write!(f, "Expected a literal"),
        }
    }
}

#[derive(Debug)]
pub struct LocalizedParseError {
    pub error: ParseError,
    pub location: SourceLocation,
}

impl Display for LocalizedParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} in file '{}' at line {}:{}",
            self.error, self.location.file_path, self.location.line, self.location.column
        )
    }
}
