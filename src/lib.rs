#![feature(box_patterns, box_syntax)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

extern crate ansi_term;

pub mod analysis;
pub mod to_javascript;
pub mod ast;
pub mod error;
pub mod parser;
pub mod compiler;
pub mod stdlib;
