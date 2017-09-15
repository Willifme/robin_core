use std::fmt;

#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

// Use a hashmap which contains an error level and a string
pub struct ErrorStack(Vec<(ErrorLevel, &'static str)>);

impl fmt::Display for ErrorStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0
            .iter()
            .map(|e| write!(f, "[{:?}] {}\n", e.0, e.1))

            // We must consume the closure
            .collect::<Vec<fmt::Result>>();

        write!(f, "")
    }
}