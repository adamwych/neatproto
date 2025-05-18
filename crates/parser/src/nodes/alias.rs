use crate::{ParseResult, Tokens};
use neatproto_ast::{Alias, Token};

pub fn parse_alias(tokens: &mut Tokens) -> ParseResult<Alias> {
    let alias_name_token = tokens.next_identifier()?;
    tokens.next_kind(Token::Equal)?;
    let aliased_type_name_token = tokens.next_identifier()?;
    tokens.next_kind(Token::Semicolon)?;

    Ok(Alias {
        alias_name: alias_name_token.value(),
        aliased_type_name: aliased_type_name_token.value(),
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::test_parser;
    use rstest::rstest;

    #[test]
    fn test_alias() {
        let alias = test_parser!(parse_alias, "foo = bar;");
        assert_eq!(&alias.alias_name, "foo");
        assert_eq!(&alias.aliased_type_name, "bar");
    }

    #[rstest]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:1")]
    #[case(";")]
    #[should_panic(expected = "Expected '=' in file 'test' at line 1:5")]
    #[case("test;")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:1")]
    #[case("123 = bar;")]
    #[should_panic(expected = "Expected an identifier in file 'test' at line 1:7")]
    #[case("bar = 123;")]
    fn test_invalid_alias(#[case] code: &str) {
        test_parser!(parse_alias, code);
    }
}
