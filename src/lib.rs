extern crate pest;

#[macro_use]
extern crate pest_derive;

extern crate ansi_term;

pub mod ast;
pub mod error;

/// For some reason, Pest cannot find the grammar file if listed in parser/mod.rs, so I listed it here
pub mod parser {
    const _GRAMMAR: &'static str = include_str!("grammar/grammar.pest");

    #[derive(Parser)]
    #[grammar = "grammar/grammar.pest"]
    pub struct ExpressionParser;
}