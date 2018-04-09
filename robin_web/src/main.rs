#[macro_use]
extern crate stdweb;

extern crate robin_core;

use std::borrow::BorrowMut;

use stdweb::web::{document, Element, IElement, IEventTarget, INonElementParentNode, INode};
use stdweb::web::event::ClickEvent;

use robin_core::compiler::Compiler;
use robin_core::parser::ExprsParser;
use robin_core::error::{Error, ErrorKind, ErrorLevel};

fn create_error_element(error: Error) -> Element {
    let error_class = match error.0 {
        ErrorLevel::Info => "info",
        ErrorLevel::Warning => "warning",
        ErrorLevel::Error => "error",
    };

    // TODO: Remove unwrap
    let element = document().create_element("li").unwrap();

    element.set_text_content(&format!("(E{}) {}", error.1 as i32, error.2));

    // TODO: Remove unwrap
    element.class_list().add(error_class).unwrap();

    element
}

fn main() {
    stdweb::initialize();

    let mut compiler = Compiler::new();

    let run = document().get_element_by_id("run").unwrap();

    run.add_event_listener(move |_: ClickEvent| {
        let lisp_input = js! {
            return lisp_input.getSession().getValue();
        };

        let compiler_list = document().get_element_by_id("compiler-list").unwrap();

        // Clear error nodes
        while compiler_list.has_child_nodes() {
            compiler_list
                .remove_child(&compiler_list.first_child().unwrap())
                .unwrap();
        }

        let parse_result = ExprsParser::new().parse(&lisp_input.as_str().unwrap());

        if parse_result.is_err() {
            // TODO: Improve parser error
            let error = Error(ErrorLevel::Error, ErrorKind::ParseError, "Parsing error".to_string());

            let error_element = create_error_element(error);

            compiler_list.append_child(&error_element);
        } else {
            let mut parse_result_unwrapped = parse_result.unwrap();

            let compiled = &compiler.borrow_mut().compile(&mut parse_result_unwrapped);

            if !compiler.errors.0.is_empty() {
                compiler.errors.0.iter().for_each(|error| {
                    compiler_list.append_child(&create_error_element(error.clone()));
                });

                // Clear the error list
                compiler.errors.0.clear();
            } else {
                js! {
                    js_output.setValue(@{compiled});

                    var newWindow = window.open("", "_blank");

                    newWindow.document.write("<script type='text/javascript'>" + @{compiled} + "</script>");
                }
            }
        }
    });

    stdweb::event_loop();
}
