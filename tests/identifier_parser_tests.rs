extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::identifier;
    use robin_core::ast::Expression;

    // I think this test is correct
    #[test]
    fn do_not_parse_a_symbol_then_an_alpha() {
        assert_ne!(identifier::identifier_literal(b"+g"),
                   IResult::Done(&b""[..], Expression::Identifier("+g".to_string())));
    }

    #[test]
    fn parse_a_left_brace() {
        assert_eq!(identifier::symbol_identifier(b"{"),
                   IResult::Done(&b""[..], "{".to_string()));
    }

    #[test]
    fn parse_a_right_brace() {
        assert_eq!(identifier::symbol_identifier(b"}"),
                   IResult::Done(&b""[..], "}".to_string()));
    }

    #[test]
    fn parse_a_left_bracket() {
        assert_eq!(identifier::symbol_identifier(b"("),
                   IResult::Done(&b""[..], "(".to_string()));
    }

    #[test]
    fn parse_a_right_bracket() {
        assert_eq!(identifier::symbol_identifier(b")"),
                   IResult::Done(&b""[..], ")".to_string()));
    }

    #[test]
    fn parse_a_left_square_bracket() {
        assert_eq!(identifier::symbol_identifier(b"["),
                   IResult::Done(&b""[..], "[".to_string()));
    }

    #[test]
    fn parse_a_right_square_bracket() {
        assert_eq!(identifier::symbol_identifier(b"]"),
                   IResult::Done(&b""[..], "]".to_string()));
    }

    #[test]
    fn parse_a_fullstop() {
        assert_eq!(identifier::symbol_identifier(b"."),
                   IResult::Done(&b""[..], ".".to_string()));
    }

    #[test]
    fn parse_a_semicolon() {
        assert_eq!(identifier::symbol_identifier(b";"),
                   IResult::Done(&b""[..], ";".to_string()));
    }

    #[test]
    fn parse_a_comma() {
        assert_eq!(identifier::symbol_identifier(b","),
                   IResult::Done(&b""[..], ",".to_string()));
    }

    #[test]
    fn parse_less_than() {
        assert_eq!(identifier::symbol_identifier(b"<"),
                   IResult::Done(&b""[..], "<".to_string()));
    }

    #[test]
    fn parse_greater_than() {
        assert_eq!(identifier::symbol_identifier(b">"),
                   IResult::Done(&b""[..], ">".to_string()));
    }

    #[test]
    fn parse_equals() {
        assert_eq!(identifier::symbol_identifier(b"="),
                   IResult::Done(&b""[..], "=".to_string()));
    }

    #[test]
    fn parse_exclamation() {
        assert_eq!(identifier::symbol_identifier(b"!"),
                   IResult::Done(&b""[..], "!".to_string()));
    }

    #[test]
    fn parse_plus() {
        assert_eq!(identifier::symbol_identifier(b"+"),
                   IResult::Done(&b""[..], "+".to_string()));
    }

    #[test]
    fn parse_minus() {
        assert_eq!(identifier::symbol_identifier(b"-"),
                   IResult::Done(&b""[..], "-".to_string()));
    }

    #[test]
    fn parse_times() {
        assert_eq!(identifier::symbol_identifier(b"*"),
                   IResult::Done(&b""[..], "*".to_string()));
    }

    #[test]
    fn parse_divide() {
        assert_eq!(identifier::symbol_identifier(b"/"),
                   IResult::Done(&b""[..], "/".to_string()));
    }

    #[test]
    fn parse_ampersand() {
        assert_eq!(identifier::symbol_identifier(b"&"),
                   IResult::Done(&b""[..], "&".to_string()));
    }

    #[test]
    fn parse_pipe() {
        assert_eq!(identifier::symbol_identifier(b"|"),
                   IResult::Done(&b""[..], "|".to_string()));
    }

    #[test]
    fn parse_power_of() {
        assert_eq!(identifier::symbol_identifier(b"^"),
                   IResult::Done(&b""[..], "^".to_string()));
    }

    #[test]
    fn parse_tilde() {
        assert_eq!(identifier::symbol_identifier(b"~"),
                   IResult::Done(&b""[..], "~".to_string()));
    }

    #[test]
    fn parse_question() {
        assert_eq!(identifier::symbol_identifier(b"?"),
                   IResult::Done(&b""[..], "?".to_string()));
    }

    #[test]
    fn parse_colon() {
        assert_eq!(identifier::symbol_identifier(b":"),
                   IResult::Done(&b""[..], ":".to_string()));
    }

    #[test]
    fn parse_multiple_symbols() {
        assert_eq!(identifier::symbol_identifier(b"|>+"),
                   IResult::Done(&b""[..], "|>+".to_string()));
    }

    #[test]
    fn parse_a_single_alpha_identifier() {
        assert_eq!(identifier::alpha_identifier(b"g"),
                   IResult::Done(&b""[..], "g".to_string()));
    }

    #[test]
    fn parse_multiple_alpha_identifiers() {
        assert_eq!(identifier::alpha_identifier(b"gbd"),
                   IResult::Done(&b""[..], "gbd".to_string()));
    }

    #[test]
    fn do_not_parse_a_symbol_and_an_alpha() {
        assert_eq!(identifier::alpha_identifier(b"gg{"),
                   IResult::Done(&b"{"[..], "gg".to_string()));
    }
}
