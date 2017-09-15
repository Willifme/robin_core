pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

// Use a hashmap which contains an error level and a string
type ErrorStack = Vec<(ErrorLevel, String)>;