use std::collections::HashMap;

use ast::Expression;
use error::Error;
use table::Table;
use to_javascript::ToJavaScript;

type Callback = fn(String, &mut [Box<Expression>], &mut Stdlib) -> Result<String, Error>;

type FunctionMap = HashMap<String, Callback>;

pub struct Stdlib<'a> {
    pub function_table: FunctionMap,
    pub variable_table: Table<'a, String>,
}

impl<'a> Stdlib<'a> {
    pub fn new(variable_table: Table<'a, String>) -> Stdlib<'a> {
        Stdlib {
            function_table: FunctionMap::new(),
            variable_table,
        }
    }

    pub fn populate(&mut self) {
        self.function_table.insert("if".to_string(), builtin_if);

        self.function_table
            .insert("return".to_string(), builtin_return);

        self.function_table
            .insert("const".to_string(), builtin_binding);

        self.function_table
            .insert("var".to_string(), builtin_binding);

        self.function_table
            .insert("let".to_string(), builtin_binding);

        // Plus and minus are both binary and unary
        // But I  have deemed binary to have a higher precedence, so binary goes first
        self.function_table.insert("+".to_string(), builtin_binop);

        self.function_table.insert("-".to_string(), builtin_binop);

        self.function_table.insert("*".to_string(), builtin_binop);

        self.function_table.insert("/".to_string(), builtin_binop);

        self.function_table.insert("%".to_string(), builtin_binop);

        self.function_table.insert("!".to_string(), builtin_unary);

        self.function_table.insert("++".to_string(), builtin_unary);

        self.function_table.insert("--".to_string(), builtin_unary);

        self.function_table.insert("~".to_string(), builtin_unary);

        self.function_table
            .insert("typeof".to_string(), builtin_unary);

        self.function_table
            .insert("delete".to_string(), builtin_unary);
    }
}

fn builtin_if(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("if statement".to_string())),
        1 => Err(Error::too_few_arguments(
            "if statement condition".to_string(),
        )),
        2 => Ok(format!(
            "if ({}) {{ {} }}",
            args[0].eval(stdlib)?,
            args[1].eval(stdlib)?
        )),
        3 => Ok(format!(
            "if ({}) {{ {} }} else {{ {} }}",
            args[0].eval(stdlib)?,
            args[1].eval(stdlib)?,
            args[2].eval(stdlib)?
        )),
        // TODO: Add error message here
        _ => panic!("Unknown number of arguments supplied to if statement"),
    }
}

fn builtin_return(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("return".to_string())),
        1 => Ok(format!("return {}", args[0].eval(stdlib)?)),
        _ => Err(Error::too_many_arguments("return".to_string())),
    }
}

fn builtin_binding(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // TODO: Add name to the error messages
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments("binding".to_string())),
        2 => {
            let (ident, value) = args.split_first_mut().unwrap();

            match ident {
                box Expression::Identifier(ref mut ident) => {
                    // TODO: Remove clones
                    stdlib
                        .variable_table
                        .insert(ident.value.clone(), ident.value.clone());

                    Ok(format!(
                        "{} {} = {}",
                        name,
                        ident.value.clone(),
                        value[0].eval(stdlib)?
                    ))
                }

                _ => Err(Error::invalid_expression(
                    "non-identifier given to binding".to_string(),
                )),
            }
        }
        _ => Err(Error::too_many_arguments("binding".to_string())),
    }
}

fn builtin_binop(
    op: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("binary operation".to_string())),
        1 => builtin_unary(op, args, stdlib),
        2 => {
            // This is messy _but_ it should make the match easier to understand
            match (&args[0], &args[1]) {
                (box Expression::Number(l), box Expression::Number(r)) => {
                    precalculate_numbers(op, l.value, r.value)
                }

                // TODO: Fix this
                (_, _) => Ok(format!(
                    "{}{}{}",
                    args[0].eval(stdlib)?,
                    op,
                    args[1].eval(stdlib)?
                )),
            }
        }
        _ => {
            let joined = args
                .into_iter()

                // TODO: Remove unwrap
                .map(|expr| expr.eval(stdlib).or_else(|e| Err(e)).unwrap())
                .collect::<Vec<String>>()
                .join(&op);

            Ok(joined)
        }
    }
}

fn builtin_unary(
    op: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match op.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", op, args[0].eval(stdlib)?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", op, args[0].eval(stdlib)?)),
        _ => Err(Error::too_few_arguments("unary operator".to_string())),
    }
}

fn precalculate_numbers(op: String, left: f64, right: f64) -> Result<String, Error> {
    match op.as_ref() {
        "+" => Ok(format!("{}", left + right)),
        "-" => Ok(format!("{}", left - right)),
        "*" => Ok(format!("{}", left * right)),
        "/" if right != 0.0 => Ok(format!("{}", left / right)),
        "%" => Ok(format!("{}", left % right)),

        "/" => Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation".to_string(),
        )),

        // Assume divide by 0 here
        _ => Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation".to_string(),
        )),
    }
}
