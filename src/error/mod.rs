#[derive(Debug)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

// Use a hashmap which contains an error level and a string
pub type ErrorStack = Vec<(ErrorLevel, &'static str)>;
