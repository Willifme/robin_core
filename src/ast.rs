use to_javascript::ToJavaScript;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    Boolean(bool),
    String(String),
    List(Vec<Box<Expression>>),

    // String = Function Name, Vec<Box<String>> = Argument List, Box<Expression> = Body
    FuncLiteral(String, Vec<Box<String>>, Box<Expression>),

    // String = Function Name, Vec<Box<Expression>> = Arguments
    FuncCall(String, Vec<Box<Expression>>),
}

/*
impl ToJavaScript for Expression {
    fn eval(&self) -> String {
        match *self {
            Expression::Number(ref val) => format!("{}", val),
            Expression::Identifier(ref val) => format!("{}", val),
            Expression::Boolean(ref val) => format!("{}", val),
            Expression::String(ref val) => val,
            Expression::List(ref vals) => {
                let values = vals
                    .into_iter()
                    .map(|e| e.eval())
                    .collect::<Vec<String>>()
                    .join(",");

                format!("[{}]", values)
            }
        }
    }
}
*/
