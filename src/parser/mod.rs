use nom::{ hex_digit, oct_digit };
use std::{ str, i32 };

// Temporarly use a f64 to represent JS numbers
type JSNumber = f64;

fn bytes_to_hex(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0x in hexadecimal, so just skip it
    i32::from_str_radix(&string[2..], 16).unwrap() as JSNumber
}

fn oct_to_hex(bytes: &[u8]) -> JSNumber {
    // TODO: Fix this unwrap
    let string = str::from_utf8(bytes).unwrap();

    // TODO: Fix this unwrap
    // Rust does not support 0o in hexadecimal, so just skip it
    i32::from_str_radix(&string[2..], 8).unwrap() as JSNumber
}

named!(pub oct_digits_literal<&[u8], JSNumber>,
    map!(
        recognize!(
            tuple!(
                alt!(tag!("0o") | tag!("0O")),
                oct_digit,
                opt!(complete!(oct_digit))
            )
        ),
        oct_to_hex
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