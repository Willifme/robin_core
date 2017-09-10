extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser;

    #[test]
    fn parse_a_single_hex_digit() {
        assert_eq!(parser::hex_digits_literal(b"0x1"), IResult::Done(&b""[..], 1.0));
    }

    #[test]
    fn parse_a_multiple_hex_digit() {
        assert_eq!(parser::hex_digits_literal(b"0x1AB"), IResult::Done(&b""[..], 427.0));
    }
}