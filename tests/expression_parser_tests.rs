extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser;
    use robin_core::ast::Expression;

    #[test]
    fn parse_multiple_expressions() {
        let exprs = Ok(vec![Expression::Boolean(true), Expression::Number(50.0)]);

        assert_eq!(parser::parse("true 50"), exprs);
    }

    #[test]
    fn booleans_should_return_a_boolean_node() {
        assert_eq!(parser::parse("true"), Ok(vec![Expression::Boolean(true)]))
    }

    #[test]
    fn identifiers_that_look_like_booleans_should_return_identifiers() {
        assert_eq!(
            parser::parse("truedog"),
            Ok(vec![Expression::Identifier("truedog".to_string())])
        )
    }

    #[test]
    fn identifiers_should_return_a_identifier_node() {
        assert_eq!(
            parser::parse("hello"),
            Ok(vec![Expression::Identifier("hello".to_string())])
        )
    }

    #[test]
    fn decimals_should_return_a_number_node() {
        assert_eq!(parser::parse("47"), Ok(vec![Expression::Number(47.0)]))
    }

    #[test]
    fn exponents_should_return_a_number_node() {
        assert_eq!(parser::parse("5e1"), Ok(vec![Expression::Number(50.0)]))
    }

    #[test]
    fn binary_should_return_a_number_node() {
        assert_eq!(parser::parse("0b1"), Ok(vec![Expression::Number(1.0)]))
    }

    #[test]
    fn octal_should_return_a_number_node() {
        assert_eq!(parser::parse("0o2"), Ok(vec![Expression::Number(2.0)]))
    }

    #[test]
    fn hexadecimal_should_return_a_number_node() {
        assert_eq!(parser::parse("0xA"), Ok(vec![Expression::Number(10.0)]))
    }

    #[test]
    fn list_should_return_a_list_node() {
        let expr = Expression::List(vec![Box::new(Expression::Boolean(true))]);

        assert_eq!(parser::parse("[true]"), Ok(vec![expr]));
    }

    #[test]
    fn function_literal_should_return_a_function_literal_node() {
        let expr = Expression::FuncLiteral(
            "example".to_string(),
            vec!["x".to_string()],
            Box::new(Expression::Boolean(true)),
        );

        assert_eq!(parser::parse("(example [x] true)"), Ok(vec![expr]));
    }

    #[test]
    fn function_literal_with_multiple_arguments() {
        let expr = Expression::FuncLiteral(
            "example".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expression::Boolean(true)),
        );

        assert_eq!(parser::parse("(example [x y] true)"), Ok(vec![expr]));
    }

    #[test]
    fn function_call_should_return_a_function_call_node() {
        let expr = Expression::FuncCall(
            Box::new(Expression::Identifier("example".to_string())),
            vec![Box::new(Expression::Boolean(true))],
        );

        assert_eq!(parser::parse("(example true)"), Ok(vec![expr]));
    }

    #[test]
    fn function_call_with_multiple_arguments() {
        let expr = Expression::FuncCall(
            Box::new(Expression::Identifier("example".to_string())),
            vec![
                Box::new(Expression::Boolean(true)),
                Box::new(Expression::Boolean(false)),
            ],
        );

        assert_eq!(parser::parse("(example true false)"), Ok(vec![expr]));
    }
}
