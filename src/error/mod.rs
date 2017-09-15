use std::fmt;
use ansi_term::Colour;

#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

pub struct Error(pub (ErrorLevel, &'static str));

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colour_level = match (self.0).0 {
            ErrorLevel::Info => Colour::Yellow.paint("Info"),

            // ansi_term does not have orange for some reason
            ErrorLevel::Warning => Colour::RGB(255, 210, 0).paint("Warning"),
            ErrorLevel::Error => Colour::Red.paint("Error"),
        };

        write!(f, "[{}] {}", colour_level, (self.0).1)
    }
}

// Use a hashmap which contains an error level and a string
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
