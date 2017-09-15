#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

// Use a hashmap which contains an error level and a string
#[derive(Debug)]
pub type ErrorStack = Vec<(ErrorLevel, &'static str)>;