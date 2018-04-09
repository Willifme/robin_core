use ansi_term::Colour;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    UndefinedVar = 0,
    TooFewArguments = 1,
    TooManyArguments = 2,
    InvalidExpression = 3,
    ParseError = 4,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error(pub ErrorLevel, pub ErrorKind, pub String);

impl Error {
    pub fn undefined_var(name: String) -> Error {
        Error(
            ErrorLevel::Error,
            ErrorKind::UndefinedVar,
            format!("Undefined var '{}'", name),
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

#[derive(Debug)]
pub struct ErrorStack(pub Vec<Error>);

impl fmt::Display for ErrorStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            // TODO: Remove this unwrap
            write!(f, "{}", error).unwrap();
        }

        write!(f, "")
    }
}
