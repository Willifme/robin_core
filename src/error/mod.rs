use std::fmt;

#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

// Use a hashmap which contains an error level and a string
pub struct ErrorStack(pub Vec<(ErrorLevel, &'static str)>);

impl fmt::Display for ErrorStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            // TODO: Remove this unwrap
            write!(f, "[{:?}] {}\n", error.0, error.1).unwrap();
        }

        write!(f, "")
    }
}
