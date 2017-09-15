use nom::alpha;
use std::str;
use ast::Expression;

fn bytes_to_string(bytes: &[u8]) -> String {
    str::from_utf8(bytes).unwrap().to_string()
}

// Convert the main parser to the AST
named!(pub identifier_literal<Expression>,
    map!(
        // Alpha must come before symbol_identifier otherwise the symbol parses
        // all the text resultin in malformed expressions
        alt!(alpha_identifier | symbol_identifier),
        Expression::Identifier
    )
);

// In Lisp, we can use symbols for identifiers so parse them as such
named!(pub symbol_identifier<String>,
    map!(
        recognize!(
            many0!(
                alt!(char!('{')
                    | char!('}')
                    | char!('(')
                    | char!(')')
                    | char!('[')
                    | char!(']')
                    | char!('.')
                    | char!(';')
                    | char!(',')
                    | char!('<')
                    | char!('>')
                    | char!('=')
                    | char!('!')
                    | char!('+')
                    | char!('-')
                    | char!('*')
                    | char!('/')
                    | char!('&')
                    | char!('|')
                    | char!('^')
                    | char!('~')
                    | char!('?')
                    | char!(':')
                )
            )
        ), bytes_to_string
    )
);

// Represents letters etc in identifiers
named!(pub alpha_identifier<String>,
    map!(alpha, bytes_to_string)
);
