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

pub fn builtin_binop(name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    // Handle unary operations where appropriate (mostly plus and minus)
    if args.len() == 1 {
        builtin_unary(name, args)

    // Handle the most common case, two expressions together e.g. (+ 1 1)
    } else if args.len() == 2 {
        Ok(format!("{}{}{}", args[0].eval()?, name, args[1].eval()?))

    // Handle the less common case of multiple expressions in one e.g. (+ 1 2 3 4 5)
    } else {
        let joined = args
            .into_iter()

            // MEGA TODO: Remove unwrap
            .map(|expr| expr.eval().unwrap())
            .collect::<Vec<String>>()
            .join(name);

        Ok(joined)
    }
}

pub fn builtin_unary(name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    match name.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", name, args[0].eval()?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", name, args[0].eval()?)),
        _ => Err(Error(ErrorLevel::Error,
                    ErrorKind::TooFewArguments,
                    "Invalid operator applied to unary operator"))
    }
}

pub fn builtin_if(_name: &String, args: &Vec<Box<Expression>>) -> Result<String, Error> {
    // TODO: Remove unwraps
    match args.len() {
        0 => {
            Err(Error(ErrorLevel::Error,
                      ErrorKind::TooFewArguments,
                      "Too few Arguments applied for if"))
        }
        1 => {
            Err(Error(ErrorLevel::Error,
                      ErrorKind::TooFewArguments,
                      "No expression applied for condition"))
        }
        2 => {
            Ok(format!("if ({}) {{ {} }}",
                       args[0].eval().unwrap(),
                       args[1].eval().unwrap()))
        }
        3 => {
            Ok(format!("if ({}) {{ {} }} else {{ {} }}",
                       args[0].eval().unwrap(),
                       args[1].eval().unwrap(),
                       args[2].eval().unwrap()))
        }
        _ => panic!("Unknown number of arguments supplied to if-statement"),
    }
}
