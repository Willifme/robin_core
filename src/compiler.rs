use analysis::table::Table;
use ast::Expression;
use error::ErrorStack;
use to_javascript::ToJavaScript;

#[derive(Debug)]
struct Compiler {
    global: Table<Expression>,
    errors: ErrorStack,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            global: Table::new(None),
            errors: ErrorStack(vec![]),
        }
    }

    fn compile(tree: &[Expression]) -> String {
        tree
            .iter()
            // TODO: Remove unwrap
            .map(|expr| expr.eval().unwrap())
            .collect::<Vec<String>>()
            .join(";")
    }
}
