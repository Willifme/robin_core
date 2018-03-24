use analysis::table::Table;
use to_javascript::ToJavaScript;
use stdlib::Stdlib;
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
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error> {
        match *self {
            Expression::Number(ref val) => Ok(format!("{}", val)),
            Expression::Identifier(ref val) =>
                if let Some(_) = stdlib.function_table.get(val) {
                    Ok(format!("{}", val))

                } else {
                    Err(Error::undefined_var("something here")) 
                }
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
                let mut stdlib = Stdlib::new(
                    Table::new(Some(Box::new(&stdlib.variable_table))));

                args
                    .into_iter()
                    .for_each(|arg| stdlib.variable_table.insert(arg.to_string(), arg.to_string()));

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
                            None =>  {
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
