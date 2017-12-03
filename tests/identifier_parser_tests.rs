extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parse_a_fullstop() {
        parses_to!(
            parser: ExpressionParser,
            input: ".",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_a_semicolon() {
        parses_to!(
            parser: ExpressionParser,
            input: ";",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_a_comma() {
        parses_to!(
            parser: ExpressionParser,
            input: ",",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_less_than() {
        parses_to!(
            parser: ExpressionParser,
            input: "<",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_greater_than() {
        parses_to!(
            parser: ExpressionParser,
            input: ">",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_equals() {
        parses_to!(
            parser: ExpressionParser,
            input: "=",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_exclamation() {
        parses_to!(
            parser: ExpressionParser,
            input: "!",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_plus() {
        parses_to!(
            parser: ExpressionParser,
            input: "+",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_minus() {
        parses_to!(
            parser: ExpressionParser,
            input: "-",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_times() {
        parses_to!(
            parser: ExpressionParser,
            input: "*",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_divide() {
        parses_to!(
            parser: ExpressionParser,
            input: "/",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_ampersand() {
        parses_to!(
            parser: ExpressionParser,
            input: "&",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_pipe() {
        parses_to!(
            parser: ExpressionParser,
            input: "|",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_power_of() {
        parses_to!(
            parser: ExpressionParser,
            input: "^",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_tilde() {
        parses_to!(
            parser: ExpressionParser,
            input: "~",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_question_mark() {
        parses_to!(
            parser: ExpressionParser,
            input: "?",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_colon() {
        parses_to!(
            parser: ExpressionParser,
            input: ":",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_multiple_symbols() {
        parses_to!(
            parser: ExpressionParser,
            input: "?+=",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 3)
            ]
        )
    }

    #[test]
    fn parse_a_single_identifier() {
        parses_to!(
            parser: ExpressionParser,
            input: "g",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 1)
            ]
        )
    }

    #[test]
    fn parse_multiple_identifiers() {
        parses_to!(
            parser: ExpressionParser,
            input: "gbd",
            rule: Rule::identifier_literal,
            tokens: [
                identifier_literal(0, 3)
            ]
        )
    }
}
