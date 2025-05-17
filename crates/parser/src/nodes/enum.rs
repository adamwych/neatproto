use crate::{LocalizedParseError, ParseError, ParseResult, Tokens};
use neatproto_ast::{BlockNode, Enum, EnumItem, Token};

pub fn parse_enum(tokens: &mut Tokens) -> ParseResult<BlockNode> {
    let name_token = tokens.next_identifier()?;
    let mut items = vec![];

    tokens.next_kind(Token::BraceOpen)?;

    let mut is_first_identifier = true;
    let mut was_previous_token_comma = false;

    while let Some(token) = tokens.next() {
        match &token.token {
            Token::Identifier(value) => {
                if !is_first_identifier && !was_previous_token_comma {
                    return Err(LocalizedParseError {
                        error: ParseError::UnexpectedToken(token.token),
                        location: token.location,
                    });
                }

                items.push(EnumItem {
                    name: value.clone(),
                });

                is_first_identifier = false;
                was_previous_token_comma = false;
            }
            Token::Comma => {
                was_previous_token_comma = true;
            }
            Token::BraceClose => {
                return Ok(BlockNode::Enum(Enum {
                    name: name_token.value(),
                    items,
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
        location: tokens.location.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;
    use crate::tests::parse;
    use rstest::rstest;

    #[test]
    fn test_enum() {
        let root_block = parse!("enum Foo { Bar, Baz }");
        match root_block.nodes.first().expect("root block is empty") {
            BlockNode::Enum(e) => {
                assert_eq!(&e.name, "Foo");

                let item_bar = e.items.get(0).expect("item #0 was not found");
                assert_eq!(&item_bar.name, "Bar");

                let item_baz = e.items.get(1).expect("item #1 was not found");
                assert_eq!(&item_baz.name, "Baz");
            }
            _ => panic!("first node is not an enum"),
        }
    }

    #[test]
    fn test_enum_with_dangling_comma() {
        parse!("enum Foo { Bar, Baz, }");
    }

    #[rstest]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:5")]
    #[case("enum")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:5")]
    #[case("enum;")]
    #[should_panic(expected = "Expected '{' in file 'test' at line 1:9")]
    #[case("enum Foo;")]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:11")]
    #[case("enum Foo {")]
    fn test_invalid_enum(#[case] code: &str) {
        parse!(code);
    }
}
