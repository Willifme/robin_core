use std::collections::HashMap;

use ast::Expression;
use analysis::table::Table;
use error::Error;
use to_javascript::ToJavaScript;

type BuiltinMap =
    HashMap<String, fn(&String, &Vec<Box<Expression>>, &Table<String>) -> Result<String, Error>>;

lazy_static! {
    pub static ref BUILTINS: BuiltinMap = {
        // The type definition is required to prevent compile errors
        let mut map: BuiltinMap = HashMap::new();

        map.insert("if".to_string(), builtin_if);

        map.insert("return".to_string(), builtin_return);

        map.insert("const".to_string(), builtin_binding);

        map.insert("var".to_string(), builtin_binding);

        map.insert("left".to_string(), builtin_binding);

        // Plus and minus are both binary and unary
        // But I  have deemed binary to have a higher precedence, so binary goes first
        map.insert("+".to_string(), builtin_binop);

        map.insert("-".to_string(), builtin_binop);

        map.insert("*".to_string(), builtin_binop);

        map.insert("/".to_string(), builtin_binop);

        map.insert("%".to_string(), builtin_binop);

        map.insert("!".to_string(), builtin_unary);

        map.insert("++".to_string(), builtin_unary);

        map.insert("--".to_string(), builtin_unary);

        map.insert("~".to_string(), builtin_unary);

        map.insert("typeof".to_string(), builtin_unary);

        map.insert("delete".to_string(), builtin_unary);

        // Don't delete this! The project will refuse to compile without it
        map
    };
}

pub fn builtin_if(_name: &String, args: &Vec<Box<Expression>>, variable_table: &Table<String>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("if statement")),
        1 => Err(Error::too_few_arguments("if statement condition")),
        2 => Ok(format!(
            "if ({}) {{ {} }}",
            args[0].eval(variable_table)?,
            args[1].eval(variable_table)?
        )),
        3 => Ok(format!(
            "if ({}) {{ {} }} else {{ {} }}",
            args[0].eval(variable_table)?,
            args[1].eval(variable_table)?,
            args[2].eval(variable_table)?
        )),
        // TODO: Add error message here
        _ => panic!("Unknown number of arguments supplied to if statement"),
    }
}

pub fn builtin_return(_name: &String, args: &Vec<Box<Expression>>, variable_table: &Table<String>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("return")),
        1 => Ok(format!("return {}", args[0].eval(variable_table)?)),
        _ => Err(Error::too_many_arguments("return")),
    }
}

pub fn builtin_binding(name: &String, args: &Vec<Box<Expression>>, variable_table: &Table<String>) -> Result<String, Error> {
    // TODO: Add name to the error messages
    match args.len() {
        0 => Err(Error::too_few_arguments("binding")),
        1 => {
            variable_table.insert(name, name.to_string());

            Ok(format!("{} {}", name, args[0].eval(variable_table)?))
        }
        _ => Err(Error::too_many_arguments("binding")),
    }
}

pub fn builtin_binop(op: &String, args: &Vec<Box<Expression>>, variable_table: &Table<String>) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("binary operation")),
        1 => builtin_unary(op, args, variable_table),
        2 => {
            // Debox and take from index
            // This is messy _but_ it should make the match easier to understand
            let &box ref left = &args[0];

            let &box ref right = &args[1];

            match (left, right) {
                (&Expression::Number(l), &Expression::Number(r)) => precalculate_numbers(op, l, r),

                (_, _) => Ok(format!("{}{}{}", left.eval(variable_table)?, op, right.eval(variable_table)?)),
            }
        }
        _ => {
            let joined = args
                .into_iter()

                // TODO: Remove unwrap
                .map(|expr| expr.eval(variable_table).or_else(|e| Err(e)).unwrap())
                .collect::<Vec<String>>()
                .join(op);

            Ok(joined)
        }
    }
}

pub fn builtin_unary(op: &String, args: &Vec<Box<Expression>>, variable_table: &Table<String>) -> Result<String, Error> {
    match op.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", op, args[0].eval(variable_table)?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", op, args[0].eval(variable_table)?)),
        _ => Err(Error::too_few_arguments("unary operator")),
    }
}

fn precalculate_numbers(op: &String, left: f64, right: f64) -> Result<String, Error> {
    match op.as_ref() {
        "+" => Ok(format!("{}", left + right)),
        "-" => Ok(format!("{}", left - right)),
        "*" => Ok(format!("{}", left * right)),
        "/" if right != 0.0 => Ok(format!("{}", left / right)),
        "%" => Ok(format!("{}", left % right)),

        "/" => Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation",
        )),

        // Assume divide by 0 here
        _ => Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation",
        )),
    }
}
