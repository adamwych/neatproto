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
    use crate::SourceFile;

    macro_rules! parse {
        ($source:expr) => {{
            let source_file = SourceFile::new_from_source("test", $source);
            match crate::parse_block(&mut source_file.tokens()) {
                Ok(block) => block,
                Err(error) => panic!("{}", error),
            }
        }};
    }

    pub(crate) use parse;

    #[test]
    fn test_empty_source() {
        let root_block = parse!("");
        assert_eq!(root_block.nodes.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Unknown identifier 'foo' in file 'test' at line 1:1")]
    fn test_unknown_identifier() {
        parse!("foo");
    }
}
