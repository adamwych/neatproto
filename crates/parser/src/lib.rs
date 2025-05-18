mod error;
mod nodes;
mod source_file;
mod tokens;

pub use error::*;
pub use nodes::*;
pub use source_file::*;
pub use tokens::*;

pub type ParseResult<T> = Result<T, LocalizedParseError>;

#[cfg(test)]
mod tests {
    macro_rules! test_parser {
        ($func:ident, $source:expr) => {{
            let source_file = crate::SourceFile::new_from_source("test", $source);
            match crate::$func(&mut source_file.tokens()) {
                Ok(block) => block,
                Err(error) => panic!("{}", error),
            }
        }};
    }

    pub(crate) use test_parser;
}
