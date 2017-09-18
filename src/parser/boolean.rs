use std::str;
use ast::Expression;

fn bytes_to_expression(bytes: &[u8]) -> Expression {
    // TODO: Remove this unwrap
    match str::from_utf8(bytes).unwrap() {
        "true" => Expression::Boolean(true), 
        "false" => Expression::Boolean(false), 

        // TODO: Add this error the error stack
        _ => panic!("Something has gone wrong with parsing boolean"),
    }
}

named!(pub boolean_literal<Expression>,
    map!(
        alt!(tag!("true") | tag!("false")),
        bytes_to_expression
    )
);
