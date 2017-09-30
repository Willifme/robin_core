extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parse_a_true_literal() {
        parses_to!(
            parser: ExpressionParser,
            input: "true",
            rule: Rule::boolean_literal,
            tokens: [
                boolean_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_false_literal() {
        parses_to!(
            parser: ExpressionParser,
            input: "false",
            rule: Rule::boolean_literal,
            tokens: [
                boolean_literal(0, 5)
            ]
        )
    }
}
