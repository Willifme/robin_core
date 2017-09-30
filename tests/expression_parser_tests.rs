extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::expression;
    use robin_core::ast::Expression;

    #[test]
    fn parse_an_identifier_bool_lookalike_suffix() {
        assert_eq!(expression::expression_literal(b"truedog"),
                   IResult::Done(&b""[..], Expression::Identifier("truedog".to_string())));
    }

    #[test]
    fn parse_an_identifier_bool_lookalike_prefix() {
        assert_eq!(expression::expression_literal(b"falsecat"),
                   IResult::Done(&b""[..], Expression::Identifier("falsecat".to_string())));
    }

    #[test]
    fn booleans_should_return_a_boolean_node() {
        assert_eq!(expression::expression_literal(b"true"),
                   IResult::Done(&b""[..], Expression::Boolean(true)));
    }
}