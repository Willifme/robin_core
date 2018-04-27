//! # Standard Library Names
//!
//! This module defines some of the names used in the standard library

// Define an array of common mathematical operations
pub static MATHS_BINOPS: &'static [&'static str] = &["+", "-", "*", "/", "%"];

// Define an array of common logical operations
pub static LOGIC_BINOPS: &'static [&'static str] =
    &["==", "===", "!=", "!==", ">", "<", ">=", "<=", "&&", "||"];

pub static UNARY_OPS: &'static [&'static str] = &["!", "++", "--", "~", "typeof", "delete"];

// Define an array of the array of some built-in generic functions. These are mostly used for the aliases
pub static GENERIC_FUNCTION: &'static [&'static str] = &[
    "console.log",
    "alert",
    "eval",
    "Array.prototype.map.call",
    "Array.prototype.forEach.call",
    "Array.prototype.filter.call",
];
