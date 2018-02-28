use std::default::Default;
use std::boxed::Box;

use ast::Expression;
use analysis::table::Table;
use error::ErrorStack;
use to_javascript::ToJavaScript;

#[derive(Debug)]
pub struct Compiler<'a> {
    pub errors: ErrorStack,
    pub variable_table: Box<Table<'a, String>>
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        Compiler {
            errors: ErrorStack(vec![]),
            variable_table: Box::new(Table::new(None)),
        }
    }

    pub fn compile(&mut self, tree: &[Expression]) -> String {
        tree
            .iter()
            // TODO: Remove unwrap
            .filter_map(|expr| {
                let result = expr.eval(&self.variable_table);

                if result.is_err() {
                    self.errors.0.push(result.clone().unwrap_err());
                }

                result.ok()
            })
            // .filter_map(|expr| {
            //     if expr.is_err() {
            //         self.errors.0.push(expr.clone().unwrap_err());
            //     }

            //     expr.ok()
            // })
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl<'a> Default for Compiler<'a> {
    fn default() -> Self {
        Self::new()
    }
}
