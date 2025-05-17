mod error;
mod source_file;
mod tokens;

pub use error::*;
pub use source_file::*;
pub use tokens::*;

use neatproto_ast::*;

pub type ParseResult<T> = Result<T, LocalizedParseError>;

pub struct Parser<'a> {
    pub tokens: Tokens<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokens<'a>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Block, LocalizedParseError> {
        let mut nodes = vec![];

        while let Some(token) = self.tokens.next() {
            nodes.push(self.parse_block_node(token)?);
        }

        Ok(Block { nodes })
    }

    fn parse_block_node(&mut self, token: LocalizedToken) -> ParseResult<BlockNode> {
        match &token.token {
            Token::Identifier(value) => match value.as_str() {
                "struct" => self.parse_structure(),
                "alias" => self.parse_alias(),
                _ => Err(LocalizedParseError {
                    error: ParseError::UnknownIdentifier(value.clone()),
                    location: token.location,
                }),
            },
            _ => Err(LocalizedParseError {
                error: ParseError::UnexpectedToken(token.token),
                location: token.location,
            }),
        }
    }

    fn parse_structure(&mut self) -> ParseResult<BlockNode> {
        let name_token = self.tokens.next_identifier()?;
        let mut fields = vec![];

        self.tokens.next_kind(Token::BraceOpen)?;
        while let Some(token) = self.tokens.next() {
            match token.token {
                Token::Identifier(value) => {
                    fields.push(self.parse_structure_field(value)?);
                }
                Token::BraceClose => {
                    return Ok(BlockNode::Structure(Structure {
                        name: name_token.value(),
                        fields,
                    }));
                }
                _ => {
                    return Err(LocalizedParseError {
                        error: ParseError::UnexpectedToken(token.token),
                        location: token.location,
                    });
                }
            }
        }

        Err(LocalizedParseError {
            error: ParseError::UnexpectedEndOfFile,
            location: self.tokens.location.clone(),
        })
    }

    fn parse_structure_field(&mut self, name: String) -> ParseResult<StructureField> {
        self.tokens.next_kind(Token::Colon)?;
        let type_name_token = self.tokens.next_identifier()?;
        self.tokens.next_kind(Token::Semicolon)?;

        Ok(StructureField {
            name,
            type_name: TypeName {
                token: type_name_token,
                resolved: None,
            },
        })
    }

    fn parse_alias(&mut self) -> ParseResult<BlockNode> {
        let alias_name_token = self.tokens.next_identifier()?;
        self.tokens.next_kind(Token::Equal)?;
        let aliased_type_name_token = self.tokens.next_identifier()?;
        self.tokens.next_kind(Token::Semicolon)?;

        Ok(BlockNode::Alias(Alias {
            alias_name: alias_name_token.value(),
            aliased_type_name: aliased_type_name_token.value(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;
    use rstest::rstest;

    macro_rules! parse {
        ($source:expr) => {{
            let source_file = SourceFile::new_from_source("test", $source);
            let mut parser = Parser::new(source_file.tokens());
            parser.parse()
        }};
    }

    fn parse_panic_if_err(code: &str) {
        if let Err(error) = parse!(code) {
            panic!("{}", error);
        }
    }

    #[test]
    fn test_empty_source() {
        let root_block = parse!("").expect("failed to parse root block");
        assert_eq!(root_block.nodes.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Unknown identifier 'foo' in file 'test' at line 1:1")]
    fn test_unknown_identifier() {
        parse_panic_if_err("foo");
    }

    #[test]
    fn test_alias() {
        let root_block = parse!("alias foo = bar;").expect("failed to parse root block");
        assert_eq!(root_block.nodes.len(), 1);
        let first_node = root_block.nodes.first().expect("root block is empty");
        match first_node {
            BlockNode::Alias(alias) => {
                assert_eq!(&alias.alias_name, "foo");
                assert_eq!(&alias.aliased_type_name, "bar");
            }
            _ => panic!("first node is not an alias"),
        }
    }

    #[rstest]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:6")]
    #[case("alias")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:6")]
    #[case("alias;")]
    #[should_panic(expected = "Expected '=' in file 'test' at line 1:11")]
    #[case("alias test;")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:7")]
    #[case("alias 123 = bar;")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:13")]
    #[case("alias bar = 123;")]
    fn test_invalid_alias(#[case] code: &str) {
        parse_panic_if_err(code);
    }

    #[test]
    fn test_structure() {
        let root_block =
            parse!("struct Foo { bar: float; baz: uint8; }").expect("failed to parse root block");
        assert_eq!(root_block.nodes.len(), 1);
        let first_node = root_block.nodes.first().expect("root block is empty");
        match first_node {
            BlockNode::Structure(structure) => {
                assert_eq!(&structure.name, "Foo");

                let field_bar = structure.fields.get(0).expect("field #0 was not found");
                assert_eq!(&field_bar.name, "bar");
                assert_eq!(&field_bar.type_name.token.value(), "float");

                let field_baz = structure.fields.get(1).expect("field #1 was not found");
                assert_eq!(&field_baz.name, "baz");
                assert_eq!(&field_baz.type_name.token.value(), "uint8");
            }
            _ => panic!("first node is not a structure"),
        }
    }

    #[rstest]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:7")]
    #[case("struct")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:7")]
    #[case("struct;")]
    #[should_panic(expected = "Expected '{' in file 'test' at line 1:11")]
    #[case("struct Foo;")]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:13")]
    #[case("struct Foo {")]
    fn test_invalid_structure(#[case] code: &str) {
        parse_panic_if_err(code);
    }

    #[rstest]
    #[should_panic(expected = "Expected ':' in file 'test' at line 1:17")]
    #[case("struct Foo { bar; }")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:18")]
    #[case("struct Foo { bar:; }")]
    #[should_panic(expected = "Expected ';' in file 'test' at line 1:23")]
    #[case("struct Foo { bar:type }")]
    fn test_invalid_structure_field(#[case] code: &str) {
        parse_panic_if_err(code);
    }
}
