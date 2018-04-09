use to_javascript::ToJavaScript;
use stdlib::Stdlib;
use error::Error;

#[derive(Debug, PartialEq)]
pub struct NumberExpression {
    pub value: f64,
}

impl NumberExpression {
    pub fn new(value: f64) -> NumberExpression {
        NumberExpression{value}
    }
}

impl ToJavaScript for NumberExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        Ok(format!("{}", self.value))
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub value: String,
}

impl IdentifierExpression {
    pub fn new(value: String) -> IdentifierExpression {
        IdentifierExpression{value}
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

#[derive(Debug, PartialEq)]
pub struct BooleanExpression {
    pub value: bool,
}

impl BooleanExpression {
    pub fn new(value: bool) -> BooleanExpression {
        BooleanExpression{value}
    }
}

impl ToJavaScript for BooleanExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        Ok(self.value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct StringExpression {
    pub value: String,
}

impl StringExpression {
    pub fn new(value: String) -> StringExpression {
        StringExpression{value}
    }
}

impl ToJavaScript for StringExpression {
    fn eval(&mut self, _stdlib: &mut Stdlib) -> Result<String, Error> {
        // TODO: Remove this clone
        Ok(self.value.clone())
    }
}

#[derive(Debug, PartialEq)]
pub struct ListExpression {
    pub value: Vec<Box<Expression>>,
    qouted: bool,
}

impl ListExpression {
    pub fn new(qouted: bool, value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression{qouted, value}
    }

    pub fn new_quoted(value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression{qouted: true, value}
    }

    pub fn new_unquoted(value: Vec<Box<Expression>>) -> ListExpression {
        ListExpression{qouted: false, value}
    }
}

impl ToJavaScript for ListExpression {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        // TODO: Remove unwrap here
        let (name, args) = self.value.split_first_mut().unwrap();

        let expr_name = name.eval(stdlib)?;

        if self.qouted {
            stdlib.function_table.get("quote").unwrap()("quote".to_string(), args, stdlib)

        } else {
            match stdlib.function_table.clone().get::<str>(&expr_name) {
                Some(func) => func(expr_name, args, stdlib),
                None => {
                    let args = args.into_iter()
                        .map(|e| e.eval(stdlib).or_else(|e| Err(e)).unwrap())
                        .collect::<Vec<String>>()
                        .join(",");

                    Ok(format!("({}({}))", expr_name, args))
                }
            }    

            // TODO: Revise this code
            /*match args[1] {
                box Expression::Identifier(ref mut ident) =>
                    if let Some(func) = stdlib.function_table.clone().get::<str>(&ident.value) {
                        func(expr_name, args, stdlib)
                    },

                _ => {
                        let args = args.into_iter()
                            .map(|e| e.eval(stdlib).or_else(|e| Err(e)).unwrap())
                            .collect::<Vec<String>>()
                            .join(",");

                        Ok(format!("({}({}))", expr_name, args))
                    },
            }*/
            // TODO: Remove to_string
            /*match (stdlib.function_table.clone().get::<str>(args[1]), box args[1]) {
                (Some(func), Expression::Identifier(ident)) => func(expr_name.clone(), args, stdlib),
                (_, _) => {
                    let args = args.into_iter()
                        .map(|e| e.eval(stdlib).or_else(|e| Err(e)).unwrap())
                        .collect::<Vec<String>>()
                        .join(",");

                    Ok(format!("({}({}))", expr_name, args))
                }
            }*/
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(NumberExpression),
    Identifier(IdentifierExpression),
    Boolean(BooleanExpression),
    String(StringExpression),
    List(ListExpression),
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

/*
#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
    List(Vec<Box<Expression>>),

    // String = Function Name, Vec<Box<String>> = Argument List, Box<Expression> = Body
    FuncLiteral(String, Vec<String>, Box<Expression>),

    // Box<Expression> = Expression being called, Vec<Box<Expression>> = Arguments
    FuncCall(Box<Expression>, Vec<Box<Expression>>),
}

impl ToJavaScript for Expression {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        match *self {
            Expression::Number(ref val) => Ok(format!("{}", val)),
            Expression::Identifier(ref val) => if let Some(_) = stdlib.function_table.get(val) {
                Ok(format!("{}", val))
            } else {
                Err(Error::undefined_var("Add undefined var here"))
            },
            Expression::Boolean(ref val) => Ok(format!("{}", val)),
            Expression::String(ref val) => Ok(val.clone()),

            Expression::List(ref mut vals) => {
                let vals = vals.into_iter()
                    // We can assume the unwrap is just a formality
                    .map(|e| e.eval(stdlib).or_else(|e| Err(e)).unwrap())
                    .collect::<Vec<String>>()
                    .join(",");

                Ok(format!("[{}]", vals))
            }

            Expression::FuncLiteral(ref name, ref args, ref mut body) => {
                let mut stdlib = Stdlib::new(Table::new(Some(Box::new(&stdlib.variable_table))));

                args.into_iter().for_each(|arg| {
                    stdlib
                        .variable_table
                        .insert(arg.to_string(), arg.to_string())
                });

                let args = args
                            .into_iter()
                            // TODO: Remove cloned
                            .cloned()
                            .map(|e| e)
                            .collect::<Vec<String>>()
                            .join(",");

                Ok(format!(
                    "function {} ({}){{ {}; }}",
                    name,
                    args,
                    body.eval(&mut stdlib)?
                ))
            }

            Expression::FuncCall(ref mut name, ref mut args) => {
                // Debox the name from Box<Expression> to Expression
                let &mut box ref mut expr_name = name;

                // TODO: Rethink this code
                match expr_name {
                    Expression::Identifier(ref ident) => {
                        // TODO: Remove clone
                        match stdlib.function_table.clone().get(&ident.clone()) {
                            Some(func) => func(ident.to_string(), args, stdlib),
                            None => {
                                let args = args.into_iter()
                                    .map(|e| e.eval(stdlib).or_else(|e| Err(e)).unwrap())
                                    .collect::<Vec<String>>()
                                    .join(",");

                                Ok(format!("({}({}))", expr_name.eval(stdlib)?, args))
                            }
                        }
                    }

                    _ => unimplemented!(),
                }
            }
        }
    }
}
*/
