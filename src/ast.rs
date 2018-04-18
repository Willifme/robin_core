use itertools::{Itertools, join};

use error::Error;
use stdlib::Stdlib;
use to_javascript::ToJavaScript;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberExpression {
    pub value: f64,
}

impl NumberExpression {
    pub fn new(value: f64) -> NumberExpression {
        NumberExpression { value }
    }
}

impl ToJavaScript for NumberExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        Ok(format!("{}", self.value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierExpression {
    pub value: String,
}

impl IdentifierExpression {
    pub fn new(value: String) -> IdentifierExpression {
        IdentifierExpression { value }
    }
}

impl ToJavaScript for IdentifierExpression {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        if let Some(_) = stdlib.variable_table.get(&self.value) {
            // TODO: Remove clone here
            Ok(self.value.clone())
        } else if let Some(_) = stdlib.function_table.get(&self.value) {
            Ok(self.value.clone())
        } else {
            // TODO: Remove clone here
            Err(Error::undefined_var(self.value.clone()))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanExpression {
    pub value: bool,
}

impl BooleanExpression {
    pub fn new(value: bool) -> BooleanExpression {
        BooleanExpression { value }
    }
}

impl ToJavaScript for BooleanExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        Ok(self.value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringExpression {
    pub value: String,
}

impl StringExpression {
    pub fn new(value: String) -> StringExpression {
        StringExpression { value }
    }
}

impl ToJavaScript for StringExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        // TODO: Remove this clone
        Ok(self.value.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListExpression {
    pub value: Vec<Box<Expression>>,
    qouted: bool,
}

impl ListExpression {
    pub fn new(qouted: bool, value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression { qouted, value }
    }

    pub fn new_quoted(value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression {
            qouted: true,
            value,
        }
    }

    pub fn new_unquoted(value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression {
            qouted: false,
            value,
        }
    }
}

impl ToJavaScript for ListExpression {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        // The expression is quoted automatically if the ' is used
        // We send all the arguments when evaluating
        if self.qouted {
            stdlib.function_table.get("quote").unwrap()("quote".to_string(), self.value.as_mut_slice(), stdlib)

        } else {
            // Create a copy of the function args for later formatting
            let mut function_args = self.value.clone();

            // TODO: Remove unwrap here
            let (name, args) = function_args.split_first_mut().unwrap();

            let expr_name = name.eval(stdlib)?;

            match (name, stdlib.function_table.clone().get::<str>(&expr_name)) {
                (box Expression::Identifier(_), Some(func)) => 
                    func(expr_name, args, stdlib),

                (box Expression::Identifier(_), _)  => {
                    let args = join(args.into_iter()
                        .map(|e| e.eval(stdlib))
                        .fold_results(vec![], |mut i, expr| {
                            i.push(expr);

                            i
                        })?, ",");

                    Ok(format!("({}({}))", expr_name, args))
                }
                _ => {
                    // TODO: Remove clone
                    let args = join(self.value.clone().into_iter()
                        .map(|mut e| e.eval(stdlib))
                        .fold_results(vec![], |mut i, expr| {
                            i.push(expr);

                            i
                        })?, ",");

                    Ok(format!("[{}]", args))
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(NumberExpression),
    Identifier(IdentifierExpression),
    Boolean(BooleanExpression),
    String(StringExpression),
    List(ListExpression),
}

impl Expression {
    /// Used to convert expressions to string
    pub fn to_string_stdlib(self, stdlib: &mut Stdlib) -> String {
        match self {
            // TODO: Clean this code up
            Expression::Number(expr) => expr.value.to_string(),
            Expression::Identifier(expr) => expr.value,
            Expression::Boolean(expr) => expr.value.to_string(),
            Expression::String(expr) => expr.value,
            Expression::List(ref list_expr) => {
                let list_fmt = list_expr
                    .value
                    .clone()
                    .into_iter()
                    .map(|expr| expr.to_string_stdlib(stdlib))
                    .collect::<Vec<String>>()
                    .join(",");

                format!("[{}]", list_fmt)
            }
        }
    }
}

impl ToJavaScript for Expression {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        match self {
            Expression::Number(expr) => expr.eval(stdlib),
            Expression::Identifier(expr) => expr.eval(stdlib),
            Expression::Boolean(expr) => expr.eval(stdlib),
            Expression::String(expr) => expr.eval(stdlib),
            Expression::List(expr) => expr.eval(stdlib),
        }
    }
}
