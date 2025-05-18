use crate::{
    LocalizedParseError, ParseError, ParseResult, Tokens, parse_alias, parse_enum, parse_structure,
};
use neatproto_ast::{Block, BlockNode, LocalizedToken, Token};

pub fn parse_block(tokens: &mut Tokens) -> Result<Block, LocalizedParseError> {
    let mut nodes = vec![];

    while let Some(token) = tokens.next() {
        nodes.push(parse_block_node(tokens, token)?);
    }

    Ok(Block { nodes })
}

pub fn parse_block_node(tokens: &mut Tokens, token: LocalizedToken) -> ParseResult<BlockNode> {
    match &token.token {
        Token::Identifier(value) => match value.as_str() {
            "struct" => parse_structure(tokens).map(BlockNode::Structure),
            "alias" => parse_alias(tokens).map(BlockNode::Alias),
            "enum" => parse_enum(tokens).map(BlockNode::Enum),
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

#[cfg(test)]
mod tests {
    use crate::tests::test_parser;
    use neatproto_ast::BlockNode;

    #[test]
    fn test_empty_source() {
        let root_block = test_parser!(parse_block, "");
        assert_eq!(root_block.nodes.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Unknown identifier 'foo' in file 'test' at line 1:1")]
    fn test_unknown_identifier() {
        test_parser!(parse_block, "foo");
    }

    #[test]
    fn test_structure_in_block() {
        let root_block = test_parser!(parse_block, "struct Foo { bar: float; baz: uint8; }");
        let first_node = root_block.nodes.first().expect("root block is empty");
        assert!(matches!(first_node, BlockNode::Structure(_)));
    }

    #[test]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:7")]
    fn test_structure_eof() {
        test_parser!(parse_block, "struct");
    }

    #[test]
    fn test_alias_in_block() {
        let root_block = test_parser!(parse_block, "alias Foo = Bar;");
        let first_node = root_block.nodes.first().expect("root block is empty");
        assert!(matches!(first_node, BlockNode::Alias(_)));
    }

    #[test]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:6")]
    fn test_alias_eof() {
        test_parser!(parse_block, "alias");
    }

    #[test]
    fn test_enum_in_block() {
        let root_block = test_parser!(parse_block, "enum Foo { Bar, Baz }");
        let first_node = root_block.nodes.first().expect("root block is empty");
        assert!(matches!(first_node, BlockNode::Enum(_)));
    }

    #[test]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:5")]
    fn test_enum_eof() {
        test_parser!(parse_block, "enum");
    }
}
