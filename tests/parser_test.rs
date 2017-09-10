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
    fn parse_a_multiple_hex_digits() {
        assert_eq!(parser::hex_digits_literal(b"0x1AB"), IResult::Done(&b""[..], 427.0));
    }

    #[test]
    fn parse_a_single_oct_digit() {
        assert_eq!(parser::oct_digits_literal(b"0o7"), IResult::Done(&b""[..], 7.0));
    }

    #[test]
    fn parse_a_multiple_oct_digits() {
        assert_eq!(parser::oct_digits_literal(b"0o67"), IResult::Done(&b""[..], 55.0));
    }

    #[test]
    fn parse_a_single_binary_digit() {
        assert_eq!(parser::binary_digits_literal(b"0b1"), IResult::Done(&b""[..], 1.0));
    }

    #[test]
    fn parse_a_multiple_binary_digits() {
        assert_eq!(parser::binary_digits_literal(b"0b011"), IResult::Done(&b""[..], 3.0));
    }
}