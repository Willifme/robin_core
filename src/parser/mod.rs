use nom::{ hex_digit, oct_digit };
use std::{ str, i32 };

// Temporarly use a f64 to represent JS numbers
type JSNumber = f64;

fn bytes_to_binary(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0b in binary, so just skip it
    i32::from_str_radix(&string[2..], 2).unwrap() as JSNumber
}

fn bytes_to_oct(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0o in hexadecimal, so just skip it
    i32::from_str_radix(&string[2..], 8).unwrap() as JSNumber
}

fn bytes_to_hex(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0x in hexadecimal, so just skip it
    i32::from_str_radix(&string[2..], 16).unwrap() as JSNumber
}

named!(pub binary_digits_literal<&[u8], JSNumber>,
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

named!(pub oct_digits_literal<&[u8], JSNumber>,
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

named!(pub hex_digits_literal<&[u8], JSNumber>,
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