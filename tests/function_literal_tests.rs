extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{Expression, IdentifierExpression, ListExpression};
    use robin_core::parser::ExprsParser;

    #[test]
    fn parses_a_function_with_no_arguments() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello () ())"), Ok(vec![expr]));
    }

    #[test]
    fn parses_a_function_with_one_argument() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "x".to_string(),
                ))),
            ]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello (x) ())"), Ok(vec![expr]));
    }

    #[test]
    fn parses_a_function_with_multiple_arguments() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "x".to_string(),
                ))),
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "y".to_string(),
                ))),
            ]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello (x y) ())"), Ok(vec![expr]));
    }
}
