use std::default::Default;

use ast::Expression;
use error::ErrorStack;
use stdlib::Stdlib;
use table::Table;
use to_javascript::ToJavaScript;

pub struct Compiler<'a> {
    pub errors: ErrorStack,
    pub stdlib: Stdlib<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        let stdlib = Stdlib::new(Table::new(None), Table::new(None), Table::new(None));

        Compiler {
            errors: ErrorStack(vec![]),
            stdlib: stdlib,
        }
    }

    pub fn compile(&mut self, tree: &mut [Expression]) -> String {
        tree
            .into_iter()
            // TODO: Remove unwrap
            .filter_map(|expr| {
                let result = expr.eval(&mut self.stdlib);

                if result.is_err() {
                    self.errors.0.push(result.clone().unwrap_err());
                }

                result.ok()
            })
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl<'a> Default for Compiler<'a> {
    fn default() -> Self {
        Self::new()
    }
}
