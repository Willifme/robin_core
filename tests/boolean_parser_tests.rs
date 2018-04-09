extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{BooleanExpression, Expression};
    use robin_core::parser::ExprsParser;

    #[test]
    fn parse_a_true_literal() {
        let expr = Expression::Boolean(BooleanExpression::new(true));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("true"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_false_literal() {
        let expr = Expression::Boolean(BooleanExpression::new(false));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("false"), Ok(vec![expr]));
    }
}
