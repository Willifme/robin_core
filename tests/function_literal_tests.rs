extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parses_a_function_with_no_arguments() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello [] [])",
            rule: Rule::function_literal,
            tokens: [
                function_literal(0, 13, [identifier_literal(1, 6), args_list(7, 9), list_literal(10, 12)])
            ]
        )
    }

    #[test]
    fn parses_a_function_with_one_argument() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello [x] [])",
            rule: Rule::function_literal,
            tokens: [
                function_literal(0, 14, [identifier_literal(1, 6), args_list(7, 10, [identifier_literal(8, 9)]), list_literal(11, 13)])
            ]
        )
    }

    #[test]
    fn parses_a_function_with_multiple_arguments() {
        parses_to!(
            parser: ExpressionParser,
            input: "(hello [x y] [])",
            rule: Rule::function_literal,
            tokens: [
                function_literal(0, 16, [identifier_literal(1, 6), args_list(7, 12, [identifier_literal(8, 9), identifier_literal(10, 11)]), list_literal(13, 15)])
            ]
        )
    }
}