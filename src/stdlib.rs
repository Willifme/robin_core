use std::collections::HashMap;

use ast::Expression;
use error::Error;
use to_javascript::ToJavaScript;

type BuiltinMap = HashMap<&'static str, fn(&String, &Vec<Box<Expression>>) -> Result<String, Error>>;

lazy_static! {
    pub static ref BUILTINS: BuiltinMap = {
        // The type definition is required to prevent compile errors
        let mut map: BuiltinMap = HashMap::new();

        map.insert("if", builtin_if);

        map.insert("return", builtin_return);

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
        0 => Err(Error::too_few_arguments("binary operation")),
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
        _ => Err(Error::too_few_arguments("unary operator")),
    }
}

pub fn builtin_if(_name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("if statement")),
        1 => Err(Error::too_few_arguments("if statement condition")),
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
        // TODO: Add error message here
        _ => panic!("Unknown number of arguments supplied to if-statement"),
    }
}

pub fn builtin_return(_name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("return")),
        1 => Ok(format!("return {}", args[0].eval()?)),
        _ => Err(Error::too_many_arguments("return")),
    }
}

fn precalculate_numbers(op: &String, left: f64, right: f64) -> Result<String, Error> {
    match op.as_ref() {
        "+" => Ok(format!("{}", left + right)),
        "-" => Ok(format!("{}", left - right)),
        "*" => Ok(format!("{}", left * right)),
        "/" if right != 0.0 => Ok(format!("{}", left / right)),
        "%" => Ok(format!("{}", left % right)),

        "/" => Err(Error::invalid_expression("Divide by zero encountered on numeric literal binary operation")),

        // Assume divide by 0 here
        _ => Err(Error::invalid_expression("Divide by zero encountered on numeric literal binary operation")),
    }
}
