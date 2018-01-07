use std::collections::HashMap;

use ast::Expression;
use error::{Error, ErrorKind, ErrorLevel};
use to_javascript::ToJavaScript;

type BuiltinMap = HashMap<&'static str, fn(&Vec<Box<Expression>>) -> Result<String, Error>>;

lazy_static! {
    pub static ref BUILTINS: BuiltinMap = {
        // The type definition is required to prevent compile errors
        let mut map: BuiltinMap = HashMap::new();

        map.insert("if", builtin_if);

        map
    };
}

pub fn builtin_if(args: &Vec<Box<Expression>>) -> Result<String, Error> {
    // TODO: Remove unwraps
    match args.len() {
        0 => {
            Err(Error(ErrorLevel::Error,
                      ErrorKind::TooFewArguments,
                      "Too few Arguments applied for if"))
        }
        1 => {
            Err(Error(ErrorLevel::Error,
                      ErrorKind::TooFewArguments,
                      "No expression applied for condition"))
        }
        2 => {
            Ok(format!("if ({}) {{ {} }}",
                       args[0].eval().unwrap(),
                       args[1].eval().unwrap()))
        }
        3 => {
            Ok(format!("if ({}) {{ {} }} else {{ {} }}",
                       args[0].eval().unwrap(),
                       args[1].eval().unwrap(),
                       args[2].eval().unwrap()))
        }
        _ => panic!("Unknown number of arguments supplied to if-statement"),
    }
}
