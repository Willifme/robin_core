extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser;
    use robin_core::ast::Expression;

    #[test]
    fn booleans_should_return_a_boolean_node() {
        assert_eq!(parser::parse("true"), Expression::Boolean(true))
    }

    #[test]
    fn identifiers_that_look_like_booleans_should_return_identifiers() {
        assert_eq!(parser::parse("truedog"), Expression::Identifier("truedog".to_string()))
    }

    #[test]
    fn identifiers_should_return_a_identifier_node() {
        assert_eq!(parser::parse("hello"), Expression::Identifier("hello".to_string()))
    }

    #[test]
    fn decimals_should_return_a_number_node() {
        assert_eq!(parser::parse("47"), Expression::Number(47.0))
    }

    #[test]
    fn exponents_should_return_a_number_node() {
        assert_eq!(parser::parse("5e1"), Expression::Number(50.0))
    }

    #[test]
    fn binary_should_return_a_number_node() {
        assert_eq!(parser::parse("0b1"), Expression::Number(1.0))
    }

    #[test]
    fn octal_should_return_a_number_node() {
        assert_eq!(parser::parse("0o2"), Expression::Number(2.0))
    }

    #[test]
    fn hexadecimal_should_return_a_number_node() {
        assert_eq!(parser::parse("0xA"), Expression::Number(10.0))
    }
}
