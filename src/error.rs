use std::fmt;
use ansi_term::Colour;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    UndefinedVar = 0,
}

#[derive(Debug, PartialEq)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Error(pub ErrorKind, pub ErrorLevel, pub &'static str);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colour_level = match self.1 {
            ErrorLevel::Info => Colour::Yellow.paint("Info"),

            // ansi_term does not have orange for some reason
            ErrorLevel::Warning => Colour::RGB(255, 210, 0).paint("Warning"),
            ErrorLevel::Error => Colour::Red.paint("Error"),
        };

        // TODO: Remove clone
        write!(f, "[{}] (E{}) {}", colour_level, self.0 as i32, self.2)
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
