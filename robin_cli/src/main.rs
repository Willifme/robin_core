#[macro_use]
extern crate clap;

extern crate rustyline;
extern crate robin_core;
extern crate itertools;

use std::process::{Command, Child, Stdio};
use std::io::{Read, Write};
use std::fs::File;

use rustyline::Editor;
use itertools::join;
use clap::{App, Arg, ArgMatches};

use robin_core::parser;
use robin_core::compiler::Compiler;

#[derive(Clone)]
struct CLI<'a> {
    args: ArgMatches<'a>
}

impl<'a> CLI<'a> {
    fn new() -> CLI<'a> {
        let app = App::new("Robin CLI")
            .name(crate_name!())
            .about(crate_description!())
            .version(crate_version!())
            .author(crate_authors!())
            .arg(Arg::with_name("repl")
                    .short("r")
                    .long("repl")
                    .help("Start the repl")
                    .takes_value(false))
            .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("input")
                    .multiple(true))
            .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("output"));

        CLI{args: app.get_matches()}
    }

    fn handle_results(self) {
        if self.args.is_present("repl") {
            Repl::new().repl()
        }

        match (self.args.values_of("input"), self.args.value_of("output")) {
            (Some(file_names), Some(output)) => {
                let mut files = vec![];

                file_names.for_each(|file| files.push(file));

                let content = self.read_files(files);

                self.compile(&content, output);
            },
            (Some(_), _) => println!("Input given without output name"),
            (_, Some(_)) => println!("Output given without input name"),
            (None, None) => {}
        }
    }

    fn read_files(&self, filenames: Vec<&str>) -> String {
        join(filenames.into_iter().map(|file_name| {
            let mut file = File::open(file_name).expect(&format!("{} not found", file_name));

            let mut content = String::new();

            file.read_to_string(&mut content).expect(&format!("Cannot read from {}", file_name));

            content
        }), "")
    }
    
    fn compile(&self, content: &str, output: &str) {
        let mut compiler = Compiler::new();

        match parser::ExprsParser::new().parse(content) {
            // TODO: Remove this unwrap
            Ok(mut expr) => { 
                let result = compiler.compile(expr.as_mut_slice());

                // TODO: Handle error stack here
                if !compiler.errors.0.is_empty() {
                    compiler.errors.0
                        .iter()
                        .for_each(|e| println!("{}", e));

                    // Clear the error stack on each input
                    compiler.errors.0.clear();

                } else {
                    self.write_files(&result, &output)
                }
            },

            Err(e) => {
                // TODO: Handle this error
                panic!("Error: {}", e);
            }
        }
    }

    fn write_files(&self, content: &str, output: &str) {
        let mut new_file = File::create(output).expect(&format!("Unable to create file: {}", output));

        new_file.write_all(content.as_bytes()).expect(&format!("Unable to write to file: {}", output));
    }
}

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
    //let mut repl = Repl::new();

    //repl.repl();

    let cli = CLI::new();

    cli.handle_results();
}
