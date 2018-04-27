//! # The Standard Library
//!
//! This module defines the standard libary used when evaluating Robin expressions
//! Most of this module contains code for converting Robin functions to special-case JS constructs

mod stdlib_names;

use itertools::{join, Itertools};

use ast::Expression;
use error::Error;
use stdlib::stdlib_names::*;
use table::Table;
use to_javascript::ToJavaScript;

/// This type is a function which contains a function name, arguments and standard library
type Callback = fn(String, &mut [Box<Expression>], &mut Stdlib) -> Result<String, Error>;

/// This struct defines the contents of the standard library.
/// function_table is a table of in-built functions
/// variable_table is a table of the variables defined by the user
/// alias_map is a table of function alises
#[derive(Clone)]
pub struct Stdlib<'a> {
    pub function_table: Table<'a, Callback>,
    pub variable_table: Table<'a, String>,
    pub alias_map: Table<'a, String>,
}

impl<'a> Stdlib<'a> {
    pub fn new(
        function_table: Table<'a, Callback>,
        variable_table: Table<'a, String>,
        alias_map: Table<'a, String>,
    ) -> Stdlib<'a> {
        let mut stdlib = Stdlib {
            function_table,
            variable_table,
            alias_map,
        };

        // The standard library must be manually populated
        stdlib.populate();

        stdlib
    }

    /// This function manually populates the pre-defined contents of the standard library
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

        // Insert each of the generic functions into the function table
        for generic in GENERIC_FUNCTION {
            self.function_table
                .insert(generic.to_string(), builtin_generic_function);
        }

        // Insert each of the alias function name into the function table
        for (builtin, _) in self.alias_map.container.iter() {
            self.function_table
                .insert(builtin.to_string(), builtin_alias);
        }

        // Insert each of the binary ops into the function table
        for binop in MATHS_BINOPS {
            self.function_table.insert(binop.to_string(), builtin_binop);
        }

        // Plus and minus are both binary and unary
        // But I have deemed binary to have a higher precedence, so binary goes first
        for logic in LOGIC_BINOPS {
            self.function_table.insert(logic.to_string(), builtin_binop);
        }

        // Insert each of the unary ops into the function table
        for unary in UNARY_OPS {
            self.function_table.insert(unary.to_string(), builtin_unary);
        }
    }
}

/// # Robin to JS transformation (If Statement)
/// Note: '=>' is used to as 'translated to'
///
/// (if) => Error: Too few arguments
/// (if true) => Error: Too few arguments
/// (if true (return 1)) => if (true) { return 1 }
/// (if true (return 1) (return 2)) => if (true) { return 1 } else { return 2 }
/// (if true (return 1) (return 3) (return 4)) => Error: too many arguments
///
fn builtin_if(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments(String::from("if statement"))),
        1 => Err(Error::too_few_arguments(String::from(
            "if statement condition",
        ))),
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
        _ => Err(Error::too_many_arguments(String::from(
            "unknown number of arguments supplied to if statement",
        ))),
    }
}

/// # Robin to JS transformation (Return statement)
/// Note: '=>' is used to as 'translated to'
///
/// (return) => Error: Too few arguments
/// (return 100) => return 100
/// (return 100 200) => Error: too many arguments
///
fn builtin_return(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments(String::from("return"))),
        1 => Ok(format!("return {}", args[0].eval(stdlib)?)),
        _ => Err(Error::too_many_arguments(String::from("return"))),
    }
}

/// # Robin to JS transformation (Const / Var / Let) statement
/// Note: '=>' is used to as 'translated to'
///
/// (const) => Error: Too few arguments
/// (const x) => Error: Too few arguments
/// (const x 100) => const x = 100
/// (const x 100 200) => Error: Too many arguments
///
fn builtin_binding(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // TODO: Add name to the error messages
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments(String::from("binding"))),
        2 => {
            let (ident, value) = args.split_first_mut().unwrap();

            // Check if the binding name is an identifier or not.
            // If an identifier, evaluate correctly.
            // If not an identifier than an Invalid Expression error is returned
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

                _ => Err(Error::invalid_expression(String::from(
                    "non-identifier given to binding",
                ))),
            }
        }
        _ => Err(Error::too_many_arguments(String::from("binding"))),
    }
}

/// # Robin to JS transformation (Generic Function)
/// Note: '=>' is used to as 'translated to'
///
/// (map (1 2 3 4) (lambda (n) (return n))) => Array.prototype.map.call([1, 2, 3, 4], (n) => { return n })
///
fn builtin_generic_function(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // Evaluate each of the results and join them with a comma
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

/// # Robin to JS transformation (Function Alias)
/// A funciton which converts an alias to the built-in function  
///
fn builtin_alias(
    name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // TODO: Remove clone
    // Get the function from the alias map
    match stdlib.alias_map.clone().get_mut(&name.clone()) {
        // If the alias is present, then call the actual function
        Some(name) => {
            stdlib.clone().function_table.get(name).unwrap()(name.to_string(), args, stdlib)
        }

        // (This should be rare)... but return an error
        _ => Err(Error::undefined_func(name)),
    }
}

/// # Robin to JS transformation (Function Definition)
/// Note: '=>' is used to as 'translated to'
///
/// (defun) => Error: Too few arguments
/// (defun example) => Error: Too few arguments
/// (defun example ()) => Error: Too few arguments
/// (defun example (n) (console.log n)) => Error: Too few arguments
/// (defun example (n) (console.log n) (console.log n)) => Error: Too many arguments
///
fn builtin_function_definition(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 | 2 => Err(Error::too_few_arguments(String::from(
            "function definition",
        ))),
        3 => {
            // First get the name of the function then the rest of the arguments
            let (name, rest) = args.split_first_mut().unwrap();

            // Then, get the functions argument then the body
            let (args, body) = rest.split_first_mut().unwrap();

            // TODO: Switch the matching conditions around for they are backwards!
            // Match the function's arguments and the function name
            match (args, name) {
                // If a list and identifier are found
                (box Expression::List(args_expr), box Expression::Identifier(func_name)) => {
                    // TODO: Remove clone
                    // Add the funciton to the parent variable table
                    stdlib
                        .variable_table
                        .insert(func_name.value.clone(), func_name.value.clone());

                    // Create a new child stdlib
                    // TODO: Remove clone
                    let mut stdlib = Stdlib::new(
                        Table::new(Some(Box::new(&stdlib.function_table))),
                        Table::new(Some(Box::new(&stdlib.variable_table))),
                        stdlib.alias_map.clone(),
                    );

                    // Conver the argument expressions to strings and join them with a comma
                    let args_fmt = join(
                        args_expr
                                .value
                                // TODO: Remove .clone
                                .clone()
                                .into_iter()
                                // TODO: Remove unwrap
                                .map(|mut expr| {
                                    // TODO: Remove unwrap
                                    let expr_name = expr.clone().to_string();

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

                // If incorrect expressions are given, we raise an error
                (_, _) => Err(Error::invalid_expression(String::from(
                    "non list given to function binding",
                ))),
            }
        }
        _ => Err(Error::too_many_arguments(String::from(
            "function definition",
        ))),
    }
}

/// # Robin to JS transformation (Quote)
/// Note: '=>' is used to as 'translated to'
/// (quote 100) => [100]
/// (quote (100)) => [[100]]
/// (quote (+ 100 1)) => [101]
///
fn builtin_quote(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    // Evaluate each of the arguments and join them with a comma
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

/// # Robin to JS transformation (Lambda)
/// Note: '=>' is used to as 'translated to'
/// (lambda) => Error: Too few arguments
/// (lambda ()) => Error: Too few arguments
/// (lambda (n) (+ n n)) => (n) => { n + n }
///
fn builtin_lambda(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments(String::from("lambda"))),
        _ => {
            // TODO: Remove unwrap
            let (args, exprs) = args.split_first_mut().unwrap();

            // Create a new child stdlib
            // TODO: Remove clone
            let mut stdlib = Stdlib::new(
                Table::new(Some(Box::new(&stdlib.function_table))),
                Table::new(Some(Box::new(&stdlib.variable_table))),
                stdlib.alias_map.clone(),
            );

            match args {
                // The first argument must be a list
                box Expression::List(list) => {
                    // Insert each argument within the list into the variable table
                    list.value.clone().into_iter().for_each(|expr| {
                        let expr_name = expr.to_string();

                        stdlib
                            .variable_table
                            .insert(expr_name.clone(), expr_name.clone());
                    });

                    // Convert each argument to a string a join them with a comma
                    let args_fmt = list.value
                        .clone()
                        .into_iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(",");

                    // Evaluate each expression after the argument and join them with a comma
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

                // If a non-list has been given
                _ => Err(Error::invalid_expression(String::from(
                    "non-list given to lambda expression",
                ))),
            }
        }
    }
}

/// # Robin to JS transformation (Raw JS)
/// Note: '=>' is used to as 'translated to'
/// (js) => Error: Too few arguments
/// (js "100") => eval("100")
/// (js "100" "200") => Error: Too many arguments
///
fn builtin_raw_js(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments(String::from("raw javascript"))),
        1 => Ok(format!("eval({})", args[0].eval(stdlib)?)),
        _ => Err(Error::too_many_arguments(String::from("raw javascript"))),
    }
}

/// # Robin to JS transformation (Nth)
/// Note: '=>' is used to as 'translated to'
/// (nth) => Error: Too few arguments
/// (nth (1 2 3 4)) => Error: Too few arguments
/// (nth (1 2 3 4) 2) => [1, 2, 3, 4][2]
/// (nth (1 2 3 4) 2 4) => Error: Too many arguments
///
fn builtin_nth(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments(String::from("nth"))),
        2 => {
            let (list, nth) = args.split_first_mut().unwrap();

            Ok(format!("{}[{}]", list.eval(stdlib)?, nth[0].eval(stdlib)?))
        }
        _ => Err(Error::too_many_arguments(String::from("nth"))),
    }
}

/// # Robin to JS transformation (Defalias)
/// Note: '=>' is used to as 'translated to'
/// (defalias) => Error: Too few arguments
/// (defalias "Array.prototype.find.call") => Error: Too few arguments
/// (defalias "Array.prototype.find.call" "find") => "find" alises to "Array.prototype.find.call", no output
/// (defalias "Array.prototype.find.call" "find" "something") => Error: Too many arguments
///
fn builtin_def_alias(
    _name: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 | 1 => Err(Error::too_few_arguments(String::from("alias"))),
        2 => {
            // Get the function name being alised
            let mut function_name = args[0].eval(stdlib)?.clone();

            // Strip the quotes from the string
            function_name.retain(|c| c != '"');

            // Get the name of the alias
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
            Ok(String::from(""))
        }
        _ => Err(Error::too_many_arguments(String::from("alias"))),
    }
}

/// # Robin to JS transformation (Binop)
/// Note: '=>' is used to as 'translated to'
/// (+) => Error: Too few arguments
/// (+ 1) => +1, Note: builtin_unary is called
/// (+ 1 1) => 1+1
/// (+ 1 1 1) => 1+1+1
///
fn builtin_binop(
    op: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match args.len() {
        0 => Err(Error::too_few_arguments(String::from("binary operation"))),
        // Handle unary functions
        1 => builtin_unary(op, args, stdlib),
        2 => {
            // This is messy _but_ it should make the match easier to understand
            match (args[0].clone(), args[1].clone()) {
                // Pre-calcuate if op an maths operation
                (box Expression::Number(ref l), box Expression::Number(ref r))
                    // If the operator is in the MATHS_BINOPS array
                    if MATHS_BINOPS.contains(&&*op) =>
                {
                    precalculate_numbers(op, l.value, r.value)
                }

                // If left and right aren't number literals, then manually format them
                (_, _) => Ok(format!(
                    "{}{}{}",
                    args[0].eval(stdlib)?,
                    op,
                    args[1].eval(stdlib)?
                )),
            }
        }
        // TODO: Consider precalculating numbers for more than one binary operation
        _ => {
            // Evaluate each expression and join them with the operator used
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

/// # Robin to JS transformation (Binop)
/// Note: '=>' is used to as 'translated to'
/// (+ 1) => +1
/// (typeof 100) => typeof 100
/// (. 100) => Error: invalid expression
///
fn builtin_unary(
    op: String,
    args: &mut [Box<Expression>],
    stdlib: &mut Stdlib,
) -> Result<String, Error> {
    match op.as_ref() {
        "+" | "-" | "!" | "++" | "--" | "~" => Ok(format!("{}{}", op, args[0].eval(stdlib)?)),

        // Unary operators which are words next an extra space.
        "typeof" | "delete" => Ok(format!("{} {}", op, args[0].eval(stdlib)?)),
        _ => Err(Error::invalid_expression(String::from("unary operator"))),
    }
}

/// Use to convert expressions contained within qoutes to a string without evaluating it
fn precalculate_numbers(op: String, left: f64, right: f64) -> Result<String, Error> {
    match op.as_ref() {
        // Perform the calculations during translation
        "+" => Ok(format!("{}", left + right)),
        "-" => Ok(format!("{}", left - right)),
        "*" => Ok(format!("{}", left * right)),
        "/" if right != 0.0 => Ok(format!("{}", left / right)),
        "%" => Ok(format!("{}", left % right)),

        "/" => Err(Error::invalid_expression(String::from(
            "Divide by zero encountered on numeric literal binary operation",
        ))),

        // Assume divide by 0 here
        _ => Err(Error::invalid_expression(String::from(
            "Divide by zero encountered on numeric literal binary operation",
        ))),
    }
}
