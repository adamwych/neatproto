use neatproto_parser::LocalizedParseError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CompileError {
    Parse(LocalizedParseError),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Parse(parse_error) => write!(f, "Parse error: {}", parse_error),
        }
    }
}
