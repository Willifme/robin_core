extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser;
    use robin_core::ast::Expression;

    #[test]
    fn booleans_should_return_a_boolean_node() {
        assert_eq!(parser::parse("true".to_string(),), Ok(Expression::Boolean(true)))
    }

    #[test]
    fn identifiers_that_look_like_booleans_should_return_identifiers() {
        assert_eq!(parser::parse("truedog".to_string(),), Ok(Expression::Identifier("truedog".to_string())))
    }

    #[test]
    fn identifiers_should_return_a_identifier_node() {
        assert_eq!(parser::parse("hello".to_string(),), Ok(Expression::Identifier("hello".to_string())))
    }

    #[test]
    fn decimals_should_return_a_number_node() {
        assert_eq!(parser::parse("47".to_string(),), Ok(Expression::Number(47.0)))
    }

    #[test]
    fn exponents_should_return_a_number_node() {
        assert_eq!(parser::parse("5e1".to_string(),), Ok(Expression::Number(50.0)))
    }

    #[test]
    fn binary_should_return_a_number_node() {
        assert_eq!(parser::parse("0b1".to_string(),), Ok(Expression::Number(1.0)))
    }

    #[test]
    fn octal_should_return_a_number_node() {
        assert_eq!(parser::parse("0o2".to_string(),), Ok(Expression::Number(2.0)))
    }

    #[test]
    fn hexadecimal_should_return_a_number_node() {
        assert_eq!(parser::parse("0xA".to_string(),), Ok(Expression::Number(10.0)))
    }
}
