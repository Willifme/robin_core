extern crate robin_core;

#[macro_use]
extern crate pest;

#[cfg(test)]
mod parser_tests {
    use robin_core::parser::{ExpressionParser, Rule};

    #[test]
    fn parse_an_empty_string() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 2)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_content() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""hello, world!""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 15)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_backspace() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\b""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_form() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\f""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_newline() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\n""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_carriage() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\r""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_horizontal_tab() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\t""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_vertical_tab() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\v""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }

    #[test]
    fn parse_a_string_with_a_null_char() {
        parses_to!(
            parser: ExpressionParser,
            input: r#""\0""#,
            rule: Rule::string_literal,
            tokens: [
                string_literal(0, 4)
            ]
        )
    }
}