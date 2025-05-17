use crate::{ParseResult, Tokens};
use neatproto_ast::{Alias, BlockNode, Token};

pub fn parse_alias(tokens: &mut Tokens) -> ParseResult<BlockNode> {
    let alias_name_token = tokens.next_identifier()?;
    tokens.next_kind(Token::Equal)?;
    let aliased_type_name_token = tokens.next_identifier()?;
    tokens.next_kind(Token::Semicolon)?;

    Ok(BlockNode::Alias(Alias {
        alias_name: alias_name_token.value(),
        aliased_type_name: aliased_type_name_token.value(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;
    use crate::tests::parse;
    use rstest::rstest;

    #[test]
    fn test_alias() {
        let root_block = parse!("alias foo = bar;");
        match root_block.nodes.first().expect("root block is empty") {
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
        parse!(code);
    }
}
