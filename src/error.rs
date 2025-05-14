use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    BadNumberOfArguments(String),
    FileError(String),
    MissingArgument(String),
    SyntaxError(String),
    UnknownInstruction(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BadNumberOfArguments(msg) => write!(f, "Wrong number of arguments: {msg}"),
            ParseError::FileError(msg) => write!(f, "File error: {msg}"),
            ParseError::MissingArgument(msg) => write!(f, "Missing argument: {msg}"),
            ParseError::SyntaxError(msg) => write!(f, "Syntax error: {msg}"),
            ParseError::UnknownInstruction(msg) => write!(f, "Unknown instruction: {msg}"),
        }
    }
}
