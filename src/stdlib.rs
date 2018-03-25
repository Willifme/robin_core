use std::collections::HashMap;

use ast::Expression;
use table::Table;
use error::Error;
use to_javascript::ToJavaScript;

type Callback = fn(String, &mut Vec<Box<Expression>>, &mut Stdlib) -> Result<String, Error>;

type FunctionMap = HashMap<String, Callback>;

pub struct Stdlib<'a> {
    pub function_table: FunctionMap,
    pub variable_table: Table<'a, String>,
}

impl<'a> Stdlib<'a> {
    pub fn new(variable_table: Table<'a, String>) -> Stdlib {
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
    args: &mut Vec<Box<Expression>>,
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("if statement")),
        1 => Err(Error::too_few_arguments("if statement condition")),
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
    args: &mut Vec<Box<Expression>>,
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("return")),
        1 => Ok(format!("return {}", args[0].eval(stdlib)?)),
        _ => Err(Error::too_many_arguments("return")),
    }
}

fn builtin_binding(
    name: String,
    args: &mut Vec<Box<Expression>>,
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // TODO: Add name to the error messages
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments("binding")),
        2 => {
            stdlib.variable_table.insert(name.clone(), name.to_string());

            // TODO: Revise this code
            let ident_string: String;

            // Open a new scope to limit borrows
            {
                match args[0] {
                    box Expression::Identifier(ref ident) => ident_string = ident.to_string(),

                    // TODO: Fix this
                    _ => {
                        panic!("Non-identifier given to binding. HOpefully this shouldn't happen.")
                    }
                };
            }

            Ok(format!(
                "{} {} = {}",
                name,
                ident_string,
                box args[1].eval(stdlib)?
            ))
        }
        _ => Err(Error::too_many_arguments("binding")),
    }
}

fn builtin_binop(
    op: String,
    args: &mut Vec<Box<Expression>>,
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("binary operation")),
        1 => builtin_unary(op, args, stdlib),
        2 => {
            // Debox and take from index
            // This is messy _but_ it should make the match easier to understand
            let &box ref left = &args[0];

            let &box ref right = &args[1];

            match (left, right) {
                (Expression::Number(l), Expression::Number(r)) => precalculate_numbers(&op, *l, *r),

                // TODO: Fix this
                _ => unimplemented!(),
                //(_, _) => Ok(format!("{}{}{}", left.eval(stdlib)?, op, right.eval(stdlib)?)),
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
    args: &mut Vec<Box<Expression>>,
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match op.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", op, args[0].eval(stdlib)?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", op, args[0].eval(stdlib)?)),
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
