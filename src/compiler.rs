use std::default::Default;

use ast::Expression;
use error::ErrorStack;
use to_javascript::ToJavaScript;

#[derive(Debug)]
pub struct Compiler {
    pub errors: ErrorStack,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            errors: ErrorStack(vec![]),
        }
    }

    pub fn compile(&mut self, tree: &[Expression]) -> String {
        tree
            .iter()
            // TODO: Remove unwrap
            .map(|expr| expr.eval())
            .filter_map(move |expr| {
                if expr.is_err() {
                    self.errors.0.push(expr.clone().unwrap_err());
                }

                expr.ok()
            })
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
