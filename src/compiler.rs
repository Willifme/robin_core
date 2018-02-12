use analysis::table::Table;
use ast::Expression;
use error::ErrorStack;
use to_javascript::ToJavaScript;

#[derive(Debug)]
pub struct Compiler {
    pub global: Table<Expression>,
    pub errors: ErrorStack,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            global: Table::new(None),
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
