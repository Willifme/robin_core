#![feature(box_patterns, box_syntax, match_default_bindings)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

extern crate ansi_term;

#[macro_use]
pub mod error;

pub mod analysis;
pub mod to_javascript;
pub mod ast;
pub mod parser;
pub mod compiler;
pub mod stdlib;
