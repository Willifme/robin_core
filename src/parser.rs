/// For some reason, Pest cannot find the grammar file if listed in parser/mod.rs
use pest::Parser;
use pest::iterators::Pair;

use ast::Expression;

use std::f64;

const _GRAMMAR: &str = include_str!("grammar/grammar.pest");

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct ExpressionParser;

fn parse_expression(input: Pair<Rule>) -> Expression {
    match input.as_rule() {
        Rule::function_literal => {
            let mut pairs = input.into_inner();

            let name = pairs
                        .next()
                        .unwrap()
                        .into_span()
                        .as_str()
                        .to_string();

            let args_list = pairs
                            .next()
                            .unwrap()
                            .into_inner()
                            .into_iter()
                            .map(|r| r.into_span().as_str().to_string())
                            .collect();

            let body = Box::new(
                            parse_expression(pairs.next().unwrap()));

            Expression::FuncLiteral(name, args_list, body)
        }

        Rule::function_call_literal => {
            let mut pairs = input.into_inner();

            let name = pairs
                        .next()
                        .unwrap()
                        .into_span()
                        .as_str()
                        .to_string();

            let args = pairs
                        .into_iter()
                        .map(|r| Box::new(parse_expression(r)))
                        .collect();

            Expression::FuncCall(name, args)
        }

        Rule::list_literal =>
            Expression::List(input
                            .into_inner()
                            .into_iter()
                            .map(|r| Box::new(parse_expression(r)))
                            .collect()),

        Rule::boolean_literal =>
            Expression::Boolean(
                input.into_span().as_str().parse::<bool>().unwrap()
            ),

        Rule::string_literal =>
            Expression::String(input.into_span().as_str().to_string()),

        Rule::identifier_literal =>
            Expression::Identifier(input.into_span().as_str().to_string()),

        // TODO: For now, just convert the value into a normal f64. No formatting!
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

        // Temporary
        _ => unreachable!()
    }

}

// TODO: Work out how to return a slice from this
pub fn parse(input: &str) -> Result<Vec<Expression>, String> {
    // TODO: Remove these unwraps
    ExpressionParser::parse(Rule::main, input)
        .map(|pairs|
             pairs
             .flat_map(|sub_pairs|
                       sub_pairs
                        .into_inner()
                       .map(|pair| parse_expression(pair)))
             .collect::<Vec<Expression>>())

        .map_err(|error| format!("{}", error))
}
