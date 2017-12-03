use std::collections::HashMap;

use ast::Expression;
use error::ErrorStack;

type SymbolTable = HashMap<String, Expression>;

#[derive(Debug)]
struct Compiler {
    global: SymbolTable,
    errors: ErrorStack,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            global: SymbolTable::new(),
            errors: ErrorStack(vec![]),
        }
    }

    fn compile(tree: Vec<Expression>) -> String {
        unimplemented!()
    }
}
