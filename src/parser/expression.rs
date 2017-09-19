use nom::IResult;
use ast::Expression;
use std::boxed::Box;

// Such imports are used to allow for the expr_literal macro to work
use parser::number::numeric_literal;
use parser::identifier::identifier_literal;
use parser::boolean::boolean_literal;

pub enum ParseResult<'a> {
    Done(Box<Expression>),
    Error(&'a [u8], Box<Expression>),
}

pub fn parse_expression(bytes: &[u8]) -> ParseResult {
    match expression_literal(bytes) {
        IResult::Done(rest, complete) => {
            if !rest.is_empty() {
                ParseResult::Error(rest, Box::new(complete))

            } else {
                ParseResult::Done(Box::new(complete))
            }
        }

        // TODO: Better error here
        _ => panic!("Parsing error here"),
    }
}

// Temp do bad stuff
named!(pub expression_literal<Expression>,
    complete!(alt!(numeric_literal | identifier_literal | boolean_literal))
);