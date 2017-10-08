/// For some reason, Pest cannot find the grammar file if listed in parser/mod.rs, so I listed it here
use pest::Parser;

use ast::Expression;

use std::f64;

const _GRAMMAR: &'static str = include_str!("grammar/grammar.pest");

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct ExpressionParser;

pub fn parse(input: &'static str) -> Expression {
    // TODO: Remove unwrap
    let mut pairs = ExpressionParser::parse_str(Rule::expression_literal, input).unwrap();

    let first = pairs.nth(0).unwrap().into_inner().nth(0).unwrap();

    match first.as_rule() {
        Rule::boolean_literal =>
            Expression::Boolean(
                first.into_span().as_str().parse::<bool>().unwrap()
            ),
        
        Rule::identifier_literal =>
            Expression::Identifier(first.into_span().as_str().to_string()),

        // TODO: For now, just conver the value into a normal f64. No formatting!
        Rule::decimal_digits_literal =>
            Expression::Number(
                first.into_span().as_str().parse::<f64>().unwrap()
            ),

        Rule::binary_digits_literal =>
            // The parse function does not support binary digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&first.into_span().as_str()[2..], 2).unwrap())
            ),

        Rule::oct_digits_literal =>
            // The parse function does not support octal digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&first.into_span().as_str()[2..], 8).unwrap())
            ),

        Rule::hex_digits_literal =>
            // The parse function does not support hexadecimal digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&first.into_span().as_str()[2..], 16).unwrap())
            ),

        // Temporary
        _ => unreachable!()
    }
}