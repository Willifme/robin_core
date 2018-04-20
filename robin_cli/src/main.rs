extern crate rustyline;
extern crate robin_core;

use std::process::{Command, Child, Stdio};
use std::io::Write;

use rustyline::Editor;

use robin_core::parser;
use robin_core::compiler::Compiler;

struct Repl<'a> {
    editor: Editor<()>,
    compiler: Compiler<'a>,
    node: Child,
}

impl<'a> Repl<'a> {
    fn new() -> Repl<'a> {
        Repl {
            editor: Editor::<()>::new(),
            compiler: Compiler::new(),
            node: Command::new("node") 
                        .stdin(Stdio::piped())
                        .stdout(Stdio::inherit())

                        // Use the --interactive flag to start the REPL despite stdin not
                        // being a terminal see node --help for more
                        .args(&["-i"])
                        .spawn()
                        .expect("Node.js not installed or found in path, please install from: https://nodejs.org/en/ and check if in path using the 'node' command")
        }
    }

    fn repl(&mut self) {
        println!("Welcome to Robin! Type code in below.");

        println!("Ignore the second prompt!");

        println!("When you run code in the REPL, you might find some Robin and some Node.js errors! Be careful");

        loop {
            let readline = self.editor.readline("> ");

            match readline {
                Ok(line) => {
                    self.parse(&line);

                    self.editor.add_history_entry(&line);
                }

                Err(_) => break, 
            }
        }
    }

    fn handle_node_repl(&mut self, input: String) {
        if let Some(ref mut stdin) = self.node.stdin.as_mut() {
            write!(stdin, "{}\n", input).unwrap();

            stdin.flush().unwrap();
        }
    }

    fn parse(&mut self, line: &str) {
        match parser::ExprsParser::new().parse(line) {
            // TODO: Remove this unwrap
            Ok(mut expr) => { 
                let output = self.compiler.compile(expr.as_mut_slice());

                // TODO: Handle error stack here
                if !self.compiler.errors.0.is_empty() {
                    self.compiler.errors.0
                        .iter()
                        .for_each(|e| println!("{}", e));

                    // Clear the error stack on each input
                    self.compiler.errors.0.clear();

                } else {
                    self.handle_node_repl(output);
                }
            },

            Err(e) => {
                // TODO: Handle this error
                panic!("Error: {}", e);
                //println!("Error");
            }
        }
    }
}

fn main() {
    let mut repl = Repl::new();

    repl.repl();
}
