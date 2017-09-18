extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::boolean;
    use robin_core::ast::Expression;

    #[test]
    fn parse_a_true_literal() {
        assert_eq!(boolean::boolean_literal(b"true"),
                   IResult::Done(&b""[..], Expression::Boolean(true)));
    }

    #[test]
    fn parse_a_false_literal() {
        assert_eq!(boolean::boolean_literal(b"false"),
                   IResult::Done(&b""[..], Expression::Boolean(false)));
    }
}