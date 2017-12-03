extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parses_a_function_call_with_no_arguments() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello)",
            rule: Rule::function_call_literal,
            tokens: [
                function_call_literal(0, 7, [identifier_literal(1, 6)]) 
            ]
        )
    }

    #[test]
    fn parses_a_function_call_with_one_argument() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello [1])",
            rule: Rule::function_call_literal,
            tokens: [
                function_call_literal(0, 11, 
                    [identifier_literal(1, 6), list_literal(7, 10, 
                        [decimal_digits_literal(8, 9)])]) 
            ]
        )
    }

    #[test]
    fn parses_a_function_call_with_two_arguments() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello [1 2])",
            rule: Rule::function_call_literal,
            tokens: [
                function_call_literal(0, 13, 
                    [identifier_literal(1, 6), list_literal(7, 12, 
                        [decimal_digits_literal(8, 9), decimal_digits_literal(10, 11)])]) 
            ]
        )
    }
}