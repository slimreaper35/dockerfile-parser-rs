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
            Self::BadNumberOfArguments(msg) => write!(f, "Wrong number of arguments: {msg}"),
            Self::FileError(msg) => write!(f, "File error: {msg}"),
            Self::MissingArgument(msg) => write!(f, "Missing argument: {msg}"),
            Self::SyntaxError(msg) => write!(f, "Syntax error: {msg}"),
            Self::UnknownInstruction(msg) => write!(f, "Unknown instruction: {msg}"),
        }
    }
}
