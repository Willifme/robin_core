#[macro_use]
extern crate stdweb;

extern crate robin_core;

use std::borrow::BorrowMut;

use stdweb::unstable::TryInto;
use stdweb::web::{IEventTarget, INode, document};
use stdweb::web::event::ClickEvent;
use stdweb::web::html_element::TextAreaElement;

use robin_core::compiler::Compiler;
use robin_core::parser::parse;

fn main() {
    stdweb::initialize();

    let mut compiler = Compiler::new();

    let input: TextAreaElement = document()
        .get_element_by_id("lisp-input")
        .unwrap()
        .first_child()
        .unwrap()
        .try_into()
        .unwrap();

    let output: TextAreaElement = document()
        .get_element_by_id("js-output")
        .unwrap()
        .first_child()
        .unwrap()
        .try_into()
        .unwrap();

    let run = document().get_element_by_id("run").unwrap();

    match parse("(+ 1 1)") {
        Ok(_) => js! {
            alert("success");
        },
        Err(_) => js! {
            alert("failure");
        }
    };

    run.add_event_listener(move |_: ClickEvent| {
        let result = js! {
            return lisp_input.getSession().getValue();
        };

        js! {
            var compiled = @{&compiler
                             .borrow_mut()
                             .compile(&parse(result.as_str().unwrap())
                                      .unwrap())};

            js_output.setValue(compiled);

            console.log(js_output);

            var newWindow = window.open("", "_blank");

            newWindow.document.write("<script type='text/javascript'>" + compiled + "</script>");
        }
    });

    stdweb::event_loop();
}
