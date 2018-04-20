use std::collections::HashMap;

pub static MATHS_BINOPS: &'static [&'static str] = &["+", "-", "*", "/", "%"];

pub static LOGIC_BINOPS: &'static [&'static str] =
    &["==", "===", "!=", "!==", ">", "<", ">=", "<=", "&&", "||"];

pub static UNARY_OPS: &'static [&'static str] = &["!", "++", "--", "~", "typeof", "delete"];

pub static GENERIC_FUNCTION: &'static [&'static str] = &[
    "console.log",
    "alert",
    "eval",
    "Array.prototype.map.call",
    "Array.prototype.forEach.call",
    "Array.prototype.filter.call",
];

lazy_static! {
    pub static ref FUNCTION_ALIAS_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        m.insert("map", "Array.prototype.map.call");

        m.insert("forEach", "Array.prototype.forEach.call");

        m.insert("filter", "Array.prototype.filter.call");

        m.insert("define", "const");

        m.insert("defun", "function");

        m.insert("not", "!");

        m.insert("and", "&&");

        m.insert("or", "||");

        m.insert("=", "===");

        m
    };
}
