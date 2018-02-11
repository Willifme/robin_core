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

    let input: TextAreaElement = document().get_element_by_id("lisp-input").unwrap().try_into().unwrap();

    let output: TextAreaElement = document().get_element_by_id("js-output").unwrap().try_into().unwrap();

    let run = document().get_element_by_id("run").unwrap();

    run.add_event_listener(move |_: ClickEvent| {
        // TODO: Handle errors
        output.set_text_content(&compiler.borrow_mut().compile(&[parse(&input.value()).unwrap()]));

        js! {
            var newWindow = window.open("", "_blank");

            newWindow.document.write("<script type='text/javascript'>" + @{output.value()} + "</script>");
        }
    });

    stdweb::event_loop();
}
