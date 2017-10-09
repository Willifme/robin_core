#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
}
