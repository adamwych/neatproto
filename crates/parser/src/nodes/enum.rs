use crate::{LocalizedParseError, ParseError, ParseResult, Tokens, parse_structure_body};
use neatproto_ast::{Enum, EnumItem, Structure, Token};

pub fn parse_enum(tokens: &mut Tokens) -> ParseResult<Enum> {
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
                    structure: None,
                    value_token: None,
                });

                is_first_identifier = false;
                was_previous_token_comma = false;
            }
            Token::Equal => {
                if let Some(last_item) = items.last_mut() {
                    last_item.value_token = Some(tokens.next_literal()?);
                } else {
                    return Err(LocalizedParseError {
                        error: ParseError::UnexpectedToken(token.token),
                        location: token.location,
                    });
                }
            }
            Token::BraceOpen => {
                if let Some(last_item) = items.last_mut() {
                    last_item.structure = Some(Structure {
                        name: last_item.name.clone(),
                        fields: parse_structure_body(tokens)?,
                    });
                } else {
                    return Err(LocalizedParseError {
                        error: ParseError::UnexpectedToken(token.token),
                        location: token.location,
                    });
                }
            }
            Token::Comma => {
                was_previous_token_comma = true;
            }
            Token::BraceClose => {
                return Ok(Enum {
                    name: name_token.value(),
                    items,
                });
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
    use crate::tests::test_parser;
    use neatproto_ast::{EnumItem, Token};
    use rstest::rstest;

    #[test]
    fn test_enum() {
        let e = test_parser!(parse_enum, "Foo { Bar = 1, Baz }");
        assert_eq!(&e.name, "Foo");

        let item_bar = e.items.get(0).expect("item #0 was not found");
        assert_eq!(&item_bar.name, "Bar");
        assert_eq!(
            item_bar
                .value_token
                .as_ref()
                .expect("item #0 should have a value")
                .token,
            Token::Digit("1".to_string())
        );

        let item_baz = e.items.get(1).expect("item #1 was not found");
        assert_eq!(&item_baz.name, "Baz");
        assert!(item_baz.value_token.is_none());
    }

    #[test]
    fn test_enum_with_dangling_comma() {
        test_parser!(parse_enum, "Foo { Bar, Baz, }");
    }

    #[test]
    fn test_tagged_union() {
        fn test_tagged_union_item(
            item: &EnumItem,
            item_name: &str,
            field_a_name: &str,
            field_a_type_name: &str,
            field_b_name: &str,
            field_b_type_name: &str,
        ) {
            assert_eq!(&item.name, item_name);
            assert!(item.value_token.is_none());

            let fields = &item.structure.as_ref().unwrap().fields;
            assert_eq!(fields.len(), 2);

            let field_a = fields.get(0).unwrap();
            assert_eq!(field_a.name, field_a_name);
            assert_eq!(
                field_a.type_name.token.token,
                Token::Identifier(field_a_type_name.into())
            );

            let field_b = fields.get(1).unwrap();
            assert_eq!(field_b.name, field_b_name);
            assert_eq!(
                field_b.type_name.token.token,
                Token::Identifier(field_b_type_name.into())
            );
        }

        let e = test_parser!(
            parse_enum,
            r#"
            Foo {
                Bar {
                    a: float;
                    b: uint32;
                },
                Baz {
                    a: uint32;
                    b: float;
                },
                Unit,
            }"#
        );
        assert_eq!(&e.name, "Foo");

        test_tagged_union_item(e.items.get(0).unwrap(), "Bar", "a", "float", "b", "uint32");
        test_tagged_union_item(e.items.get(1).unwrap(), "Baz", "a", "uint32", "b", "float");
    }

    #[rstest]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:1")]
    #[case(";")]
    #[should_panic(expected = "Expected '{' in file 'test' at line 1:4")]
    #[case("Foo;")]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:6")]
    #[case("Foo {")]
    #[should_panic(expected = "Expected a literal in file 'test' at line 1:13")]
    #[case("Foo { Bar = \"strings-not-allowed\" }")]
    fn test_invalid_enum(#[case] code: &str) {
        test_parser!(parse_enum, code);
    }
}
