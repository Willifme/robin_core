extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{BooleanExpression, Expression, IdentifierExpression, ListExpression,
                          NumberExpression};
    use robin_core::parser::ExprsParser;

    #[test]
    fn parse_an_empty_list() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("()"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_list_with_one_value() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![Box::new(
            Expression::Identifier(IdentifierExpression::new("hello".to_string())),
        )]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello)"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_list_with_multiple_values() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::Boolean(BooleanExpression::new(true))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(hello true)"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_list_with_symbols() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![Box::new(
            Expression::Identifier(IdentifierExpression::new("+".to_string())),
        )]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(+)"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_list_with_multiple_symbols() {
        let expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "+".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(1 as f64))),
            Box::new(Expression::Number(NumberExpression::new(1 as f64))),
        ]));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("(+ 1 1)"), Ok(vec![expr]));
    }
}
