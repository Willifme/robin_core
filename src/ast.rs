use analysis::table::VARIABLE_TABLE;
use to_javascript::ToJavaScript;
use stdlib::BUILTINS;
use error::Error;

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
    fn eval(&self) -> Result<String, Error> {
        match *self {
            Expression::Number(ref val) => Ok(format!("{}", val)),
            Expression::Identifier(ref val) =>
                if let Some(_) = VARIABLE_TABLE.get(val as &str) {
                    Ok(format!("{}", val))

                } else {
                    Err(Error::undefined_var("")) 
                }
            Expression::Boolean(ref val) => Ok(format!("{}", val)),
            Expression::String(ref val) => Ok(val.clone()),

            Expression::List(ref vals) => {
                let vals = vals.into_iter()
                    // We can assume the unwrap is just a formality
                    .map(|e| e.eval().or_else(|e| Err(e)).unwrap())
                    .collect::<Vec<String>>()
                    .join(",");

                Ok(format!("[{}]", vals))
            }

            Expression::FuncLiteral(ref name, ref args, ref body) => {
                let args = args
                            .into_iter()
                            // TODO: Remove clone
                            .cloned()
                            .map(|e| e)
                            .collect::<Vec<String>>()
                            .join(",");

                Ok(format!(
                    "function {} ({}){{ {}; }}",
                    name,
                    args,
                    body.eval()?
                ))
            }

            Expression::FuncCall(ref name, ref args) => {
                // Debox the name from Box<Expression> to Expression
                let &box ref expr_name = name;

                // TODO: Rethink this code
                match *expr_name {
                    Expression::Identifier(ref ident) => {
                        // We unwrap, but it should be okay
                        if let Some(func) = BUILTINS.get(ident as &str) {
                            func(ident, args)
                        } else {
                            let args = args.into_iter()
                                .map(|e| e.eval().or_else(|e| Err(e)).unwrap())
                                .collect::<Vec<String>>()
                                .join(",");

                            // TODO: Remove unwrap()
                            Ok(format!("({}({}))", name.eval().unwrap(), args))
                        }
                    }

                    _ => unimplemented!(),
                }
            }
        }
    }
}
