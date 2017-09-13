extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser::number;

    #[test]
    fn parse_a_single_hex_digit() {
        assert_eq!(number::hex_digits_literal(b"0x1"),
                   IResult::Done(&b""[..], 1.0));
    }

    #[test]
    fn parse_multiple_hex_digits() {
        assert_eq!(number::hex_digits_literal(b"0x1AB"),
                   IResult::Done(&b""[..], 427.0));
    }

    #[test]
    fn parse_a_single_oct_digit() {
        assert_eq!(number::oct_digits_literal(b"0o7"),
                   IResult::Done(&b""[..], 7.0));
    }

    #[test]
    fn parse_multiple_oct_digits() {
        assert_eq!(number::oct_digits_literal(b"0o67"),
                   IResult::Done(&b""[..], 55.0));
    }

    #[test]
    fn parse_a_single_binary_digit() {
        assert_eq!(number::binary_digits_literal(b"0b1"),
                   IResult::Done(&b""[..], 1.0));
    }

    #[test]
    fn parse_multiple_binary_digits() {
        assert_eq!(number::binary_digits_literal(b"0b011"),
                   IResult::Done(&b""[..], 3.0));
    }

    #[test]
    fn parse_a_single_decimal_digit() {
        assert_eq!(number::decimal_digits_literal(b"5"),
                   IResult::Done(&b""[..], 5.0));
    }

    #[test]
    fn parse_multiple_decimal_digits() {
        assert_eq!(number::decimal_digits_literal(b"55"),
                   IResult::Done(&b""[..], 55.0));
    }

    #[test]
    fn parse_a_single_lower_exponent_digit() {
        assert_eq!(number::exponent_part_digits_literal(b"e6"),
                   IResult::Done(&b""[..], 6.0));
    }

    #[test]
    fn parse_multiple_single_digits() {
        assert_eq!(number::exponent_part_digits_literal(b"e66"),
                   IResult::Done(&b""[..], 66.0));
    }

    #[test]
    fn parse_a_single_upper_exponent_digit() {
        assert_eq!(number::exponent_part_digits_literal(b"E7"),
                   IResult::Done(&b""[..], 7.0));
    }

    #[test]
    fn parse_multiple_upper_digits() {
        assert_eq!(number::exponent_part_digits_literal(b"E77"),
                   IResult::Done(&b""[..], 77.0));
    }
}
