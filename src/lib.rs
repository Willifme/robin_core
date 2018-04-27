#![feature(box_patterns, box_syntax, match_default_bindings)]

///! # Library module
///!
///! This module is the main library for Robin.
extern crate ansi_term;

extern crate itertools;

extern crate lalrpop_util;

#[macro_use]
pub mod error;

pub mod ast;
pub mod compiler;
pub mod parser;
pub mod stdlib;
pub mod table;
pub mod to_javascript;
