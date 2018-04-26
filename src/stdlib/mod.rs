mod stdlib_names;

use itertools::{join, Itertools};

use ast::Expression;
use error::Error;
use stdlib::stdlib_names::*;
use table::Table;
use to_javascript::ToJavaScript;

type Callback = fn(String, &mut [Box<Expression>], &mut Stdlib) -> Result<String, Error>;

#[derive(Clone)]
pub struct Stdlib<'a> {
    pub function_table: Table<'a, Callback>,
    pub variable_table: Table<'a, String>,
    pub alias_map: Table<'a, String>,
}

impl<'a> Stdlib<'a> {
    pub fn new(function_table: Table<'a, Callback>, variable_table: Table<'a, String>, alias_map: Table<'a, String>) -> Stdlib<'a> {
        let mut stdlib = Stdlib {
            function_table,
            variable_table,
            alias_map,
        };

        stdlib.populate();

        stdlib
    }

    fn populate(&mut self) {
        self.function_table.insert(String::from("if"), builtin_if);

        self.function_table
            .insert(String::from("return"), builtin_return);

        self.function_table
            .insert(String::from("const"), builtin_binding);

        self.function_table
            .insert(String::from("var"), builtin_binding);

        self.function_table
            .insert(String::from("let"), builtin_binding);

        self.function_table
            .insert(String::from("function"), builtin_function_definition);

        self.function_table
            .insert(String::from("quote"), builtin_quote);

        self.function_table
            .insert(String::from("lambda"), builtin_lambda);

        self.function_table
            .insert(String::from("js"), builtin_raw_js);

        self.function_table.insert(String::from("nth"), builtin_nth);

        self.function_table
            .insert(String::from("defalias"), builtin_def_alias);

        self.alias_map.insert(
            String::from("map"),
            String::from("Array.prototype.map.call"),
        );

        self.alias_map.insert(
            String::from("forEach"),
            String::from("Array.prototype.forEach.call"),
        );

        self.alias_map.insert(
            String::from("filter"),
            String::from("Array.prototype.filter.call"),
        );

        self.alias_map
            .insert(String::from("define"), String::from("const"));

        self.alias_map
            .insert(String::from("defun"), String::from("function"));

        self.alias_map
            .insert(String::from("not"), String::from("!"));

        self.alias_map
            .insert(String::from("and"), String::from("&&"));

        self.alias_map
            .insert(String::from("or"), String::from("||"));

        self.alias_map
            .insert(String::from("="), String::from("==="));

        for generic in GENERIC_FUNCTION {
            self.function_table
                .insert(generic.to_string(), builtin_generic_function);
        }

        for (builtin, _) in self.alias_map.container.iter() {
            self.function_table
                .insert(builtin.to_string(), builtin_alias);
        }

        for binop in MATHS_BINOPS {
            self.function_table.insert(binop.to_string(), builtin_binop);
        }

        // Plus and minus are both binary and unary
        // But I have deemed binary to have a higher precedence, so binary goes first
        for logic in LOGIC_BINOPS {
            self.function_table.insert(logic.to_string(), builtin_binop);
        }

        for unary in UNARY_OPS {
            self.function_table.insert(unary.to_string(), builtin_unary);
        }
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
        _ => Err(Error::too_many_arguments(
            "unknown number of arguments supplied to if statement".to_string(),
        )),
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
fn builtin_generic_function(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    let args_fmt = join(
        args.into_iter()
            .map(|expr| expr.eval(stdlib))
            .fold_results(vec![], |mut i, expr| {
                i.push(expr);

                i
            })?,
        ",",
    );

    Ok(format!("{}({})", name, args_fmt))
}

fn builtin_alias(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // TODO: Remove clone
    match stdlib.alias_map.clone().get_mut(&name.clone()) {
        Some(name) => {
            stdlib.clone().function_table.get(name).unwrap()(name.to_string(), args, stdlib)
        }

        _ => Err(Error::undefined_func(name)),
    }
}

fn builtin_function_definition(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 | 2 => Err(Error::too_few_arguments("function definition".to_string())),
        3 => {
            let (name, rest) = args.split_first_mut().unwrap();

            let (args, body) = rest.split_first_mut().unwrap();

            match (args, name) {
                (box Expression::List(args_expr), box Expression::Identifier(func_name)) => {
                    // TODO: Remove clone
                    // Add the funciton to the parent variable table
                    stdlib
                        .variable_table
                        .insert(func_name.value.clone(), func_name.value.clone());

                    // Create a new child stdlib
                    // TODO: Remove clone
                    let mut stdlib =
                        Stdlib::new(
                            Table::new(Some(Box::new(&stdlib.function_table))), 
                            Table::new(Some(Box::new(&stdlib.variable_table))), 
                            stdlib.alias_map.clone());

                    let args_fmt = join(
                        args_expr
                                .value
                                // TODO: Remove .clone
                                .clone()
                                .into_iter()
                                // TODO: Remove unwrap
                                .map(|mut expr| {
                                    // TODO: Remove unwrap
                                    let expr_name = identifier_to_string(expr.clone()).unwrap();

                                    // TODO: Remove clones
                                    stdlib.variable_table.insert(expr_name.clone(),
                                        expr_name.clone());

                                    expr.eval(&mut stdlib)
                                })
                                .fold_results(vec![], |mut i, expr| {
                                    i.push(expr);

                                    i
                                })?,
                        ",",
                    );

                    Ok(format!(
                        "function {}({}){{ {}; }}",
                        func_name.value,
                        args_fmt,
                        body[0].eval(&mut stdlib)?
                    ))
                }
                (_, _) => Err(Error::invalid_expression(
                    "non list given to function binding".to_string(),
                )),
            }
        }
        _ => Err(Error::too_many_arguments("function definition".to_string())),
    }
}

fn builtin_quote(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    let args_fmt = join(
        args
        .into_iter()
        // TODO: Remove clone
        .map(|expr| expr.eval(stdlib))
        .fold_results(vec![], |mut i, expr| {
            i.push(expr);

            i
        })?,
        ",",
    );

    Ok(format!("\"[{}]\"", args_fmt))
}

fn builtin_lambda(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments("lambda".to_string())),
        _ => {
            // TODO: Remove unwrap
            let (args, exprs) = args.split_first_mut().unwrap();

            // Create a new child stdlib
            // TODO: Remove clone
            let mut stdlib = Stdlib::new(
                Table::new(Some(Box::new(&stdlib.function_table))),
                Table::new(Some(Box::new(&stdlib.variable_table))), 
                stdlib.alias_map.clone());

            match args {
                box Expression::List(list) => {
                    list.value.clone().into_iter().for_each(|expr| {
                        // TODO: Remove unwrap
                        //let expr_name = identifier_to_string(expr.clone()).unwrap();

                        let expr_name = expr.to_string();

                        stdlib
                            .variable_table
                            .insert(expr_name.clone(), expr_name.clone());
                    });

                    let args_fmt = list.value
                        .clone()
                        .into_iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(",");

                    let exprs_fmt = join(
                        exprs
                            .into_iter()
                            .map(|expr| expr.eval(&mut stdlib))
                            .fold_results(vec![], |mut i, expr| {
                                i.push(expr);

                                i
                            })?,
                        ",",
                    );

                    Ok(format!("({}) => {{ {} }}", args_fmt, exprs_fmt))
                }
                _ => Err(Error::invalid_expression(
                    "non-list given to lambda expression".to_string(),
                )),
            }
        }
    }
}

fn builtin_raw_js(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments("raw javascript".to_string())),
        _ => {
            let js_fmt = join(
                args.into_iter()
                    .map(|expr| expr.eval(stdlib))
                    .fold_results(vec![], |mut i, expr| {
                        i.push(expr);

                        i
                    })?,
                ";",
            );

            Ok(format!("eval({})", js_fmt))
        }
    }
}

fn builtin_nth(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments("nth".to_string())),
        2 => {
            let (list, nth) = args.split_first_mut().unwrap();

            Ok(format!("{}[{}]", list.eval(stdlib)?, nth[0].eval(stdlib)?))
        }
        _ => Err(Error::too_many_arguments("nth".to_string())),
    }
}

fn builtin_def_alias(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments("alias".to_string())),
        2 => {
            let mut function_name = args[0].eval(stdlib)?.clone();

            // Strip the quotes from the string
            function_name.retain(|c| c != '"');

            let mut alias = args[1].eval(stdlib)?.clone();

            // Strip the quotes from the string
            alias.retain(|c| c != '"');

            // We insert the alias into the map
            stdlib
                .alias_map
                .insert(alias.clone(), function_name.clone());

            // We add the alias into the function
            stdlib.function_table.insert(alias.clone(), builtin_alias);

            // The function being alised needs to be added as a generic function
            stdlib
                .function_table
                .insert(function_name.clone(), builtin_generic_function);

            // The function doesn't actually produce any output!
            Ok("".to_string())
        }
        _ => Err(Error::too_many_arguments("alias".to_string())),
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
            match (args[0].clone(), args[1].clone()) {
                // Pre-calcuate if op an maths operation
                (box Expression::Number(ref l), box Expression::Number(ref r))
                    if MATHS_BINOPS.contains(&&*op) =>
                {
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
            let joined = join(
                args.into_iter()
                    .map(|expr| expr.eval(stdlib))
                    .fold_results(vec![], |mut i, expr| {
                        i.push(expr);

                        i
                    })?,
                &op,
            );

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

fn identifier_to_string(expr: Box<Expression>) -> Option<String> {
    match expr {
        box Expression::Identifier(ident) => Some(ident.value),
        _ => None,
    }
}

/// Use to convert expressions contained within qoutes to a string without evaluating it
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
