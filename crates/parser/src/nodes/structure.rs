use crate::{LocalizedParseError, ParseError, ParseResult, Tokens};
use neatproto_ast::{Structure, StructureField, Token, TypeName};

pub fn parse_structure(tokens: &mut Tokens) -> ParseResult<Structure> {
    let name_token = tokens.next_identifier()?;
    let mut fields = vec![];

    tokens.next_kind(Token::BraceOpen)?;
    while let Some(token) = tokens.next() {
        match token.token {
            Token::Identifier(value) => {
                fields.push(parse_structure_field(tokens, value)?);
            }
            Token::BraceClose => {
                return Ok(Structure {
                    name: name_token.value(),
                    fields,
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

pub fn parse_structure_field(tokens: &mut Tokens, name: String) -> ParseResult<StructureField> {
    tokens.next_kind(Token::Colon)?;
    let type_name_token = tokens.next_identifier()?;

    let mut is_array = false;
    let mut array_size = None;

    let next_token = tokens.next_or_err()?;
    match next_token.token {
        Token::SquareOpen => {
            is_array = true;

            let next_next_token = tokens.next_or_err()?;
            match next_next_token.token {
                Token::Digit(_) => {
                    array_size = Some(next_next_token);
                    tokens.next_kind(Token::SquareClose)?;
                    tokens.next_kind(Token::Semicolon)?;
                }
                Token::SquareClose => {
                    tokens.next_kind(Token::Semicolon)?;
                }
                _ => {
                    return Err(LocalizedParseError {
                        error: ParseError::UnexpectedToken(next_next_token.token),
                        location: next_next_token.location,
                    });
                }
            }
        }
        Token::Semicolon => {}
        _ => {
            return Err(LocalizedParseError {
                error: ParseError::UnexpectedToken(next_token.token),
                location: next_token.location,
            });
        }
    }

    Ok(StructureField {
        name,
        type_name: TypeName {
            token: type_name_token,
            is_array,
            array_size,
            resolved: None,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::test_parser;
    use neatproto_ast::Token;
    use rstest::rstest;

    #[test]
    fn test_structure() {
        let structure = test_parser!(
            parse_structure,
            "Foo { bar: float; baz: uint8; fixedBars: float[4]; dynamicBars: float[]; }"
        );
        assert_eq!(&structure.name, "Foo");

        let field_bar = structure.fields.get(0).expect("field #0 was not found");
        assert_eq!(&field_bar.name, "bar");
        assert_eq!(&field_bar.type_name.token.value(), "float");

        let field_baz = structure.fields.get(1).expect("field #1 was not found");
        assert_eq!(&field_baz.name, "baz");
        assert_eq!(&field_baz.type_name.token.value(), "uint8");

        let field_fixed_bars = structure.fields.get(2).expect("field #2 was not found");
        assert_eq!(&field_fixed_bars.name, "fixedBars");
        assert_eq!(&field_fixed_bars.type_name.token.value(), "float");
        assert_eq!(field_fixed_bars.type_name.is_array, true);
        assert_eq!(
            field_fixed_bars
                .type_name
                .array_size
                .as_ref()
                .unwrap()
                .token,
            Token::Digit("4".into())
        );

        let field_dynamic_bars = structure.fields.get(3).expect("field #3 was not found");
        assert_eq!(&field_dynamic_bars.name, "dynamicBars");
        assert_eq!(&field_dynamic_bars.type_name.token.value(), "float");
        assert_eq!(field_dynamic_bars.type_name.is_array, true);
        assert!(field_dynamic_bars.type_name.array_size.is_none());
    }

    #[rstest]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:1")]
    #[case(";")]
    #[should_panic(expected = "Expected '{' in file 'test' at line 1:4")]
    #[case("Foo;")]
    #[should_panic(expected = "Unexpected end of file in file 'test' at line 1:6")]
    #[case("Foo {")]
    fn test_invalid_structure(#[case] code: &str) {
        test_parser!(parse_structure, code);
    }

    #[rstest]
    #[should_panic(expected = "Expected ':' in file 'test' at line 1:10")]
    #[case("Foo { bar; }")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:11")]
    #[case("Foo { bar:; }")]
    #[should_panic(expected = "Unexpected token '}' in file 'test' at line 1:16")]
    #[case("Foo { bar:type }")]
    fn test_invalid_structure_field(#[case] code: &str) {
        test_parser!(parse_structure, code);
    }
}
