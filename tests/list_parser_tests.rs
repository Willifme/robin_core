extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parse_an_empty_list() {
        parses_to!(
            parser: ExpressionParser,
            input: "[]",
            rule: Rule::list_literal,
            tokens: [
                list_literal(0, 2)
            ]
        )
    }

    #[test]
    fn parse_a_list_with_one_value() {
        parses_to!(
            parser: ExpressionParser,
            input: "[hello]",
            rule: Rule::list_literal,
            tokens: [
                list_literal(0, 7, [identifier_literal(1, 6)])
            ]
        )
    }

    #[test]
    fn parse_a_list_with_multiple_values() {
        parses_to!(
            parser: ExpressionParser,
            input: "[hello true]",
            rule: Rule::list_literal,
            tokens: [
                list_literal(0, 12, [identifier_literal(1, 6), boolean_literal(7, 11)])
            ]
        )
    }

    #[test]
    fn parse_a_list_with_symbols() {
        parses_to!(
            parser: ExpressionParser,
            input: "[+]",
            rule: Rule::list_literal,
            tokens: [
                list_literal(0, 3, [identifier_literal(1, 2)])
            ]
        )
    }

    #[test]
    fn parse_a_list_with_multiple_symbols() {
        parses_to!(
            parser: ExpressionParser,
            input: "[+ 1 1]",
            rule: Rule::list_literal,
            tokens: [
                list_literal(0, 7, 
                    [identifier_literal(1, 2), decimal_digits_literal(3, 4), decimal_digits_literal(5, 6)]
                )
            ]
        )
    }
}