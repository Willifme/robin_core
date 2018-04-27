//! # To JavaScript
//!
//! This module defines the trait for translating Robin code to JavaScript

use error::Error;
use stdlib::Stdlib;

pub trait ToJavaScript {
    fn eval(&mut self, stdlib: &mut Stdlib) -> Result<String, Error>;
}
