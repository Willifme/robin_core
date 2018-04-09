extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::ExprsParser;
    use robin_core::ast::{Expression, IdentifierExpression};

    #[test]
    fn parse_a_fullstop() {
        let expr = Expression::Identifier(IdentifierExpression::new(".".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("."), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_semicolon() {
        let expr = Expression::Identifier(IdentifierExpression::new(";".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(";"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_comma() {
        let expr = Expression::Identifier(IdentifierExpression::new(",".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(","), Ok(vec![expr]));
    }

    #[test]
    fn parse_less_than() {
        let expr = Expression::Identifier(IdentifierExpression::new("<".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("<"), Ok(vec![expr]));
    }

    #[test]
    fn parse_greater_than() {
        let expr = Expression::Identifier(IdentifierExpression::new(">".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(">"), Ok(vec![expr]));
    }

    #[test]
    fn parse_equals() {
        let expr = Expression::Identifier(IdentifierExpression::new("=".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("="), Ok(vec![expr]));
    }

    #[test]
    fn parse_exclamation() {
        let expr = Expression::Identifier(IdentifierExpression::new("!".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("!"), Ok(vec![expr]));
    }

    #[test]
    fn parse_plus() {
        let expr = Expression::Identifier(IdentifierExpression::new("+".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("+"), Ok(vec![expr]));
    }

    #[test]
    fn parse_minus() {
        let expr = Expression::Identifier(IdentifierExpression::new("-".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("-"), Ok(vec![expr]));
    }

    #[test]
    fn parse_times() {
        let expr = Expression::Identifier(IdentifierExpression::new("*".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("*"), Ok(vec![expr]));
    }

    #[test]
    fn parse_divide() {
        let expr = Expression::Identifier(IdentifierExpression::new("/".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("/"), Ok(vec![expr]));
    }

    #[test]
    fn parse_ampersand() {
        let expr = Expression::Identifier(IdentifierExpression::new("&".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("&"), Ok(vec![expr]));
    }

    #[test]
    fn parse_pipe() {
        let expr = Expression::Identifier(IdentifierExpression::new("|".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("|"), Ok(vec![expr]));
    }

    #[test]
    fn parse_power_of() {
        let expr = Expression::Identifier(IdentifierExpression::new("^".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("^"), Ok(vec![expr]));
    }

    #[test]
    fn parse_tilde() {
        let expr = Expression::Identifier(IdentifierExpression::new("~".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("~"), Ok(vec![expr]));
    }

    #[test]
    fn parse_question_mark() {
        let expr = Expression::Identifier(IdentifierExpression::new("?".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("?"), Ok(vec![expr]));
    }

    #[test]
    fn parse_colon() {
        let expr = Expression::Identifier(IdentifierExpression::new(":".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(":"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_symbols() {
        let expr = Expression::Identifier(IdentifierExpression::new("?+=".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("?+="), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_identifier() {
        let expr = Expression::Identifier(IdentifierExpression::new("g".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("g"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_identifiers() {
        let expr = Expression::Identifier(IdentifierExpression::new("gbd".to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("gbd"), Ok(vec![expr]));
    }
}
