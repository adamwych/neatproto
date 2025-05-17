use crate::{LocalizedParseError, ParseError, ParseResult, Tokens, parse_alias, parse_structure};
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
            "struct" => parse_structure(tokens),
            "alias" => parse_alias(tokens),
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
