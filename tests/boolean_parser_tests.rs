extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::boolean;

    #[test]
    fn parse_a_true_literal() {
        assert_eq!(boolean::boolean_literal(b"true"),
                   IResult::Done(&b""[..], true));
    }

    #[test]
    fn parse_a_false_literal() {
        assert_eq!(boolean::boolean_literal(b"false"),
                   IResult::Done(&b""[..], false));
    }
}