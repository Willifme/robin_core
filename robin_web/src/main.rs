#[macro_use]
extern crate stdweb;

extern crate robin_core;

use std::borrow::BorrowMut;

use stdweb::web::{IEventTarget, INode, document};
use stdweb::web::event::ClickEvent;

use robin_core::compiler::Compiler;
use robin_core::parser::parse;

fn main() {
    stdweb::initialize();

    let mut compiler = Compiler::new();

    let run = document().get_element_by_id("run").unwrap();

    run.add_event_listener(move |_: ClickEvent| {
        let lisp_input = js! {
            return lisp_input.getSession().getValue();
        };

        let compiler_list = document() 
                .get_element_by_id("compiler-list")
                .unwrap();

        let error_element = document()
            .create_element("li")
            .unwrap();

        // Clear error nodes
        while compiler_list.has_child_nodes() {
            compiler_list
                .remove_child(&compiler_list.first_child().unwrap())
                .unwrap();
        }

        let parse_result = parse(&lisp_input.as_str().unwrap());

        if parse_result.is_err() {
            error_element
                .set_text_content(
                    &format!("{}", parse_result.err().unwrap()));

            compiler_list.append_child(&error_element);
            
        } else {
            let parse_result_unwrapped = parse_result.unwrap();

            let compiled = &compiler
                .borrow_mut()
                .compile(&parse_result_unwrapped);

            compiler
                .errors.0
                .iter()
                .for_each(|error| {
                    error_element
                        .set_text_content(
                            &format!("{}", error));
                    compiler_list.append_child(&error_element);
                });

            js! {
                js_output.setValue(@{compiled});

                var newWindow = window.open("", "_blank");

                //newWindow.document.write("<script type='text/javascript'>" + @{compiled} + "</script>");
            }
        }
    });

    stdweb::event_loop();
}
