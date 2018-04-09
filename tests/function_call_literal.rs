extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::ExprsParser;
    use robin_core::ast::{Expression, ListExpression, IdentifierExpression, NumberExpression};

    #[test]
    fn parses_a_function_call_with_no_arguments() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(
                IdentifierExpression::new("hello".to_string()))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello)"), Ok(vec![expr]));
    }

    #[test]
    fn parses_a_function_call_with_one_argument() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(
                IdentifierExpression::new("hello".to_string()))),
            Box::new(Expression::Number(
                NumberExpression::new(1.0)))
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello 1)"), Ok(vec![expr]));
    }
    #[test]
    fn parses_a_function_call_with_two_arguments() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(
                IdentifierExpression::new("hello".to_string()))),
            Box::new(Expression::Number(NumberExpression::new(1.0))),
            Box::new(Expression::Number(NumberExpression::new(2.0))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello 1 2)"), Ok(vec![expr]));
    }
}
