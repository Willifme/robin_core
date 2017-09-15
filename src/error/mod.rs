use std::fmt;

#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

pub struct Error(pub (ErrorLevel, &'static str));

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {}", (self.0).0, (self.0).1)
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
