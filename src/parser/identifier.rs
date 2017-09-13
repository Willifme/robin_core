use nom::alpha;
use std::str;

fn bytes_to_string(bytes: &[u8]) -> String {
    str::from_utf8(bytes).unwrap().to_string()
}

named!(pub identifier_literal<String>,
    alt!(symbol_identifier | alpha_identifier)
);

// In Lisp, we can use symbols for identifiers so parse them as such
named!(pub symbol_identifier<String>,
    map!(
        recognize!(
            many0!(
                alt!(tag!("{")
                    | tag!("}")
                    | tag!("(")
                    | tag!(")")
                    | tag!("[")
                    | tag!("]")
                    | tag!(".")
                    | tag!(";")
                    | tag!(",")
                    | tag!("<")
                    | tag!(">")
                    | tag!("=")
                    | tag!("!")
                    | tag!("+")
                    | tag!("-")
                    | tag!("*")
                    | tag!("/")
                    | tag!("&")
                    | tag!("|")
                    | tag!("^")
                    | tag!("~")
                    | tag!("?")
                    | tag!(":")
                )
            )
        ), bytes_to_string
    )
);

// Represents letters etc in identifiers
named!(pub alpha_identifier<String>, 
    map!(
        recognize!(
            tuple!(
                alpha,
                // Somethimes a symbol might follow a letter
                opt!(symbol_identifier)
            )
        ), 
        bytes_to_string
    )
);
