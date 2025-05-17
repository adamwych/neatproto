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
    tokens.next_kind(Token::Semicolon)?;

    Ok(StructureField {
        name,
        type_name: TypeName {
            token: type_name_token,
            resolved: None,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::SourceFile;
    use crate::tests::parse;
    use rstest::rstest;
    use neatproto_ast::BlockNode;

    #[test]
    fn test_structure() {
        let root_block = parse!("struct Foo { bar: float; baz: uint8; }");
        match root_block.nodes.first().expect("root block is empty") {
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
        parse!(code);
    }

    #[rstest]
    #[should_panic(expected = "Expected ':' in file 'test' at line 1:17")]
    #[case("struct Foo { bar; }")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:18")]
    #[case("struct Foo { bar:; }")]
    #[should_panic(expected = "Expected ';' in file 'test' at line 1:23")]
    #[case("struct Foo { bar:type }")]
    fn test_invalid_structure_field(#[case] code: &str) {
        parse!(code);
    }
}
