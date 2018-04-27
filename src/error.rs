///! # Compiler Errors
///!
///! This module defines the errors created by the compiler
use ansi_term::Colour;
use std::fmt;

/// Define an enum for the different types of errors detected.
/// Each error has a number assoicated with it for reference
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    UndefinedVar = 0,
    UndefinedFunc = 1,
    TooFewArguments = 2,
    TooManyArguments = 3,
    InvalidExpression = 4,
    ParseError = 5,
}

/// Define an enum for the different levels of error which can occur
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

/// Define a tuple enum containing an error level, error kind and error message
#[derive(Debug, Clone, PartialEq)]
pub struct Error(pub ErrorLevel, pub ErrorKind, pub String);

/// Define a series of functions which are shortcuts for commonly occuring errors
impl Error {
    pub fn undefined_var(name: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::UndefinedVar,
            format!("Undefined var '{}'", name),
        )
    }

    pub fn undefined_func(name: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::UndefinedFunc,
            format!("Undefined func '{}'", name),
        )
    }

    pub fn too_few_arguments(construct: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::TooFewArguments,
            format!("Too few arguments applied for {}", construct),
        )
    }

    pub fn too_many_arguments(construct: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::TooManyArguments,
            format!("Too many arguments applied for {}", construct),
        )
    }

    pub fn invalid_expression(construct: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::InvalidExpression,
            format!("{}", construct),
        )
    }
}

impl fmt::Display for Error {
    // Add colour to different error levels
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colour_level = match self.0 {
            ErrorLevel::Info => Colour::Yellow.paint("Info"),

            // ansi_term does not have orange for some reason
            ErrorLevel::Warning => Colour::RGB(255, 210, 0).paint("Warning"),
            ErrorLevel::Error => Colour::Red.paint("Error"),
        };

        // TODO: Remove clone
        write!(f, "[{}] (E{}) {}", colour_level, self.1 as i32, self.2)
    }
}

// Define a struct which contains a vector of elements (used as a stack)
#[derive(Debug)]
pub struct ErrorStack(pub Vec<Error>);

impl fmt::Display for ErrorStack {
    // Iterate over each element and print it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            // TODO: Remove this unwrap
            write!(f, "{}", error).unwrap();
        }

        write!(f, "")
    }
}
