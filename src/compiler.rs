///! # Compiler
///!
///! This module defines the struct for the Compiler.
///! The compiler struct acts like an interface for the CLI and web editor to use
use ast::Expression;
use error::ErrorStack;
use stdlib::Stdlib;
use table::Table;
use to_javascript::ToJavaScript;

/// This is the main struct for interacting with the Robin translator
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

    // This function takes a tree from the parser and translates it
    pub fn compile(&mut self, tree: &mut [Expression]) -> String {
        // Iterate over each node from the AST and translate it
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
