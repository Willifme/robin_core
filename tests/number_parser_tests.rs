
extern crate robin_core;
#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parse_a_single_hex_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "0x1",
            rule: Rule::hex_digits_literal,
            tokens: [
                hex_digits_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_multiple_hex_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "0x1AB",
            rule: Rule::hex_digits_literal,
            tokens: [
                hex_digits_literal(0, 5)
            ]
        )
    }

    #[test]
    fn parse_a_single_oct_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "0o7",
            rule: Rule::oct_digits_literal,
            tokens: [
                oct_digits_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_multiple_oct_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "0o67",
            rule: Rule::oct_digits_literal,
            tokens: [
                oct_digits_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_single_binary_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "0b1",
            rule: Rule::binary_digits_literal,
            tokens: [
                binary_digits_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_multiple_binary_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "0b011",
            rule: Rule::binary_digits_literal,
            tokens: [
                binary_digits_literal(0, 5)
            ]
        )
    }

    #[test]
    fn parse_a_single_decimal_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "5",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 1)
            ]
        )
    }


    #[test]
    fn parse_multiple_decimal_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "55",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 2)
            ]
        )
    }

    #[test]
    fn parse_a_single_lower_exponent_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "5e6",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_multiple_lower_exponent_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "55e66",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 5)
            ]
        )
    }

    #[test]
    fn parse_a_single_upper_exponent_digit() {
        parses_to!(
            parser: ExpressionParser,
            input: "3E7",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_multiple_upper_exponent_digits() {
        parses_to!(
            parser: ExpressionParser,
            input: "66E77",
            rule: Rule::decimal_digits_literal,
            tokens: [
                decimal_digits_literal(0, 5)
            ]
        )
    }
}