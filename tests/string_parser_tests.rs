extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{Expression, StringExpression};
    use robin_core::parser::ExprsParser;

    #[test]
    fn parse_an_empty_string() {
        let expr = Expression::String(StringExpression::new(r#""""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_content() {
        let expr = Expression::String(StringExpression::new(r#""hello, world!""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""hello, world!""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_backspace() {
        let expr = Expression::String(StringExpression::new(r#""\b""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\b""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_form() {
        let expr = Expression::String(StringExpression::new(r#""\f""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\f""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_newline() {
        let expr = Expression::String(StringExpression::new(r#""\n""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\n""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_carriage() {
        let expr = Expression::String(StringExpression::new(r#""\r""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\r""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_horizontal_tab() {
        let expr = Expression::String(StringExpression::new(r#""\t""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\t""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_vertical_tab() {
        let expr = Expression::String(StringExpression::new(r#""\v""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\v""#), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_string_with_a_null_char() {
        let expr = Expression::String(StringExpression::new(r#""\0""#.to_string()));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse(r#""\0""#), Ok(vec![expr]));
    }
}
