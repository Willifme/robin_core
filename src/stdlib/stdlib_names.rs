pub static MATHS_BINOPS: &'static [&'static str] = &["+", "-", "*", "/", "%"];

pub static LOGIC_BINOPS: &'static [&'static str] =
    &["==", "===", "!=", "!==", ">", "<", ">=", "<=", "&&", "||"];

pub static UNARY_OPS: &'static [&'static str] = &["!", "++", "--", "~", "typeof", "delete"];

pub static GENERIC_FUNCTION: &'static [&'static str] = 
    &["console.log", "alert", "eval", "Array.prototype.map.call", "Array.prototype.forEach.call", "Array.prototype.filter.call",];