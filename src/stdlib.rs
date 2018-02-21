use std::collections::HashMap;

use ast::Expression;
use error::{Error, ErrorKind, ErrorLevel};
use to_javascript::ToJavaScript;

type BuiltinMap = HashMap<&'static str, fn(&String, &Vec<Box<Expression>>) -> Result<String, Error>>;

lazy_static! {
    pub static ref BUILTINS: BuiltinMap = {
        // The type definition is required to prevent compile errors
        let mut map: BuiltinMap = HashMap::new();

        map.insert("if", builtin_if);

        // Plus and minus are both binary and unary
        // But I  have deemed binary to have a higher precedence, so binary goes first
        map.insert("+", builtin_binop);

        map.insert("-", builtin_binop);

        map.insert("*", builtin_binop);

        map.insert("/", builtin_binop);

        map.insert("%", builtin_binop);

        map.insert("!", builtin_unary);

        map.insert("++", builtin_unary);

        map.insert("--", builtin_unary);

        map.insert("~", builtin_unary);

        map.insert("typeof", builtin_unary);

        map.insert("delete", builtin_unary);

        // Don't delete this! The project will refuse to compile without it
        map
    };
}

pub fn builtin_binop(op: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error(
            ErrorLevel::Error,
            ErrorKind::TooFewArguments,
            "Too few Arguments applied for binary operation",
        )),
        1 => builtin_unary(op, args),
        2 => {
            // Debox and take from index
            // This is messy _but_ it should make the match easier to understand
            let &box ref left = &args[0];

            let &box ref right = &args[1];

            match (left, right) {
                (&Expression::Number(l), &Expression::Number(r)) =>
                    precalculate_numbers(op, l, r),

                (_, _) => Ok(format!("{}{}{}", left.eval()?, op, right.eval()?)),
            }
        },
        _ => {
            let joined = args
                .into_iter()

                // TODO: Remove unwrap
                .map(|expr| expr.eval().or_else(|e| Err(e)).unwrap())
                .collect::<Vec<String>>()
                .join(op);

            Ok(joined)
        },
    }
}

pub fn builtin_unary(op: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    match op.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", op, args[0].eval()?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", op, args[0].eval()?)),
        _ => Err(Error(ErrorLevel::Error,
                    ErrorKind::TooFewArguments,
                    "Invalid operator applied to unary operator"))
    }
}

pub fn builtin_if(_name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    // TODO: Remove unwraps
    match args.len() {
        0 => Err(Error(
            ErrorLevel::Error,
            ErrorKind::TooFewArguments,
            "Too few Arguments applied for if",
        )),
        1 => Err(Error(
            ErrorLevel::Error,
            ErrorKind::TooFewArguments,
            "No expression applied for condition",
        )),
        2 => Ok(format!(
            "if ({}) {{ {} }}",
            args[0].eval()?,
            args[1].eval()?
        )),
        3 => Ok(format!(
            "if ({}) {{ {} }} else {{ {} }}",
            args[0].eval()?,
            args[1].eval()?,
            args[2].eval()?
        )),
        _ => panic!("Unknown number of arguments supplied to if-statement"),
    }
}

fn precalculate_numbers(op: &String, left: f64, right: f64) -> Result<String, Error> {
    match op.as_ref() {
        "+" => Ok(format!("{}", left + right)),
        "-" => Ok(format!("{}", left - right)),
        "*" => Ok(format!("{}", left * right)),
        "/" if right != 0.0 => Ok(format!("{}", left / right)),
        "%" => Ok(format!("{}", left % right)),

        // Assume divide by 0 here
        "/" => Err(Error(ErrorLevel::Error,
                    ErrorKind::InvalidExpression,
                    "Divide by zero encountered on numeric literal binary operation")),
        _ => Err(Error(ErrorLevel::Error,
                    ErrorKind::InvalidExpression,
                    "Invalid operator given to numeric binary operation"))
    }
}
