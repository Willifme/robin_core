extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{Expression, NumberExpression};
    use robin_core::parser::ExprsParser;

    #[test]
    fn parse_a_single_hex_digit() {
        let expr = Expression::Number(NumberExpression::new(0x1 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0x1"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_hex_digits() {
        let expr = Expression::Number(NumberExpression::new(0x1AB as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0x1AB"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_oct_digit() {
        let expr = Expression::Number(NumberExpression::new(0o7 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0o7"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_oct_digits() {
        let expr = Expression::Number(NumberExpression::new(0o67 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0o67"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_binary_digit() {
        let expr = Expression::Number(NumberExpression::new(0b1 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0b1"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_binary_digits() {
        let expr = Expression::Number(NumberExpression::new(0b011 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("0b011"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_decimal_digit() {
        let expr = Expression::Number(NumberExpression::new(5.0));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("5"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_decimal_digits() {
        let expr = Expression::Number(NumberExpression::new(55.0));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("55"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_lower_exponent_digit() {
        let expr = Expression::Number(NumberExpression::new(5e6 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("5e6"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_lower_exponent_digits() {
        let expr = Expression::Number(NumberExpression::new(55e66 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("55e66"), Ok(vec![expr]));
    }

    #[test]
    fn parse_a_single_upper_exponent_digit() {
        let expr = Expression::Number(NumberExpression::new(3E7 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("3E7"), Ok(vec![expr]));
    }

    #[test]
    fn parse_multiple_upper_exponent_digits() {
        let expr = Expression::Number(NumberExpression::new(66E77 as f64));

        let parser = ExprsParser::new();

        assert_eq!(parser.parse("66E77"), Ok(vec![expr]));
    }
}
