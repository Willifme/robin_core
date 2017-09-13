extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::identifier;

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
}