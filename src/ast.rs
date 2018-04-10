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
