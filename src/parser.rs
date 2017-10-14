/// For some reason, Pest cannot find the grammar file if listed in parser/mod.rs, so I listed it here
use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StrInput;

use ast::Expression;

use std::f64;

const _GRAMMAR: &'static str = include_str!("grammar/grammar.pest");

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct ExpressionParser;

fn parse_expression(input: Pair<Rule, StrInput>) -> Expression {
    match input.as_rule() {
        Rule::boolean_literal =>
            Expression::Boolean(
                input.into_span().as_str().parse::<bool>().unwrap()
            ),

        Rule::string_literal => 
            Expression::String(format!("{}", input.into_span().as_str())
            ),

        Rule::identifier_literal =>
            Expression::Identifier(input.into_span().as_str().to_string()),

        // TODO: For now, just conver the value into a normal f64. No formatting!
        Rule::decimal_digits_literal =>
            Expression::Number(
                input.into_span().as_str().parse::<f64>().unwrap()
            ),

        Rule::binary_digits_literal =>
            // The parse function does not support binary digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&input.into_span().as_str()[2..], 2).unwrap())
            ),

        Rule::oct_digits_literal =>
            // The parse function does not support octal digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&input.into_span().as_str()[2..], 8).unwrap())
            ),

        Rule::hex_digits_literal =>
            // The parse function does not support hexadecimal digits, so do it the hard way
            Expression::Number(
                f64::from(i32::from_str_radix(&input.into_span().as_str()[2..], 16).unwrap())
            ),

        Rule::list_literal => 
            Expression::List(input
                            .into_inner()
                            .into_iter()
                            .map(|r| Box::new(parse_expression(r)))
                            .collect()),

        // Temporary
        _ => unreachable!()
    }

}

pub fn parse(input: String) -> Result<Expression, String> {
    // TODO: Remove unwrap
    match ExpressionParser::parse_str(Rule::main, &input) {
        Ok(mut pair) =>
            Ok(parse_expression(pair.nth(0).unwrap().into_inner().nth(0).unwrap())),
        
        Err(e) => Err(format!("{}", e))
    }
}