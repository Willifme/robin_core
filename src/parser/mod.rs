use nom::{ digit, hex_digit, oct_digit };
use std::{ str, i32 };

// Temporarly use a f64 to represent JS numbers
type JSNumber = f64;

// TODO: Implement proper parsing (and saving) of the number literal.

fn bytes_to_exponent(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support e or E in exponents, so just skip it
    f64::from(i32::from_str_radix(&string[1..], 10).unwrap())
}

fn bytes_to_digit(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    //i32::from_str_radix(&string, 10).unwrap() as JSNumber
    string.parse::<JSNumber>().unwrap()
}

fn bytes_to_binary(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0b in binary, so just skip it
    f64::from(i32::from_str_radix(&string[2..], 2).unwrap())
}

fn bytes_to_oct(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0o in hexadecimal, so just skip it
    f64::from(i32::from_str_radix(&string[2..], 8).unwrap())
}

fn bytes_to_hex(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0x in hexadecimal, so just skip it
    f64::from(i32::from_str_radix(&string[2..], 16).unwrap())
}

named!(pub numeric_literals<JSNumber>,
    alt!(hex_digits_literal 
        | oct_digits_literal 
        | binary_digits_literal 
        | decimal_digits_literal 
        | exponent_part_digits_literal)
);

named!(pub exponent_part_digits_literal<JSNumber>,
    map!(
        recognize!(
            tuple!(
                alt!(tag!("e") | tag!("E")),
                decimal_digits_literal
            )
        ),
        bytes_to_exponent
    )
);

// Support for negating and un-negating numbers will be supplied
// by a function within the language, so we don't parse it
named!(pub decimal_digits_literal<JSNumber>, 
    map!(
        digit,
        bytes_to_digit
    )
);

named!(pub binary_digits_literal<JSNumber>,
    map!(
        recognize!(
            tuple!(
                alt!(tag!("0b") | tag!("0B")),
                many1!(binary_digit)
            )
        ),
        bytes_to_binary
    )
);

named!(binary_digit, alt!(tag!("0") | tag!("1")));

named!(pub oct_digits_literal<JSNumber>,
    map!(
        recognize!(
            tuple!(
                alt!(tag!("0o") | tag!("0O")),
                oct_digit,
                opt!(complete!(oct_digit))
            )
        ),
        bytes_to_oct
    )
);

named!(pub hex_digits_literal<JSNumber>,
    map!(
        recognize!(
            tuple!(
                alt!(tag!("0x") | tag!("0X")),
                hex_digit,
                opt!(complete!(hex_digit))
            )
        ),
        bytes_to_hex 
    )
);