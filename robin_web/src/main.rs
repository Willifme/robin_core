///! # Robin-Web
///!
///! This module defines how the translator should interact with the web page 

#[macro_use]
extern crate stdweb;

extern crate robin_core;

use std::borrow::BorrowMut;

use stdweb::web::event::ClickEvent;
use stdweb::web::{document, Element, IElement, IEventTarget, INode, INonElementParentNode};

use robin_core::compiler::Compiler;
use robin_core::error::{Error, ErrorKind, ErrorLevel};
use robin_core::parser::ExprsParser;

/// A function which creates an error web page element from an error struct
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

    // Create the compiler
    let mut compiler = Compiler::new();

    // Get the run button
    let run = document().get_element_by_id("run").unwrap();

    // Add a click event to the element
    run.add_event_listener(move |_: ClickEvent| {
        // Get the Robin 
        let lisp_input = js! {
            return lisp_input.getSession().getValue();
        };

        // Get the list of elements
        let compiler_list = document().get_element_by_id("compiler-list").unwrap();

        // Clear error nodes
        while compiler_list.has_child_nodes() {
            compiler_list
                .remove_child(&compiler_list.first_child().unwrap())
                .unwrap();
        }

        // Parse the lisp input
        let parse_result = ExprsParser::new().parse(&lisp_input.as_str().unwrap());

        if parse_result.is_err() {
            // TODO: Improve parser error
            let error = Error(
                ErrorLevel::Error,
                ErrorKind::ParseError,
                "Parsing error".to_string(),
            );

            // Create a new element from the error
            let error_element = create_error_element(error);

            // Add the new element
            compiler_list.append_child(&error_element);

        } else {
            let mut parse_result_unwrapped = parse_result.unwrap();

            let compiled = &compiler.borrow_mut().compile(&mut parse_result_unwrapped);

            // Add the compiler errors
            if !compiler.errors.0.is_empty() {
                compiler.errors.0.iter().for_each(|error| {
                    compiler_list.append_child(&create_error_element(error.clone()));
                });

                // Clear the error list
                compiler.errors.0.clear();
            } else {
                // Open a new window with the compiled code
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
