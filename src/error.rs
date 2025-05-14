use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    FileError(String),
    SyntaxError(String),
    UnknownInstruction(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FileError(msg) => write!(f, "File error: {msg}"),
            ParseError::SyntaxError(msg) => write!(f, "Syntax error: {msg}"),
            ParseError::UnknownInstruction(msg) => write!(f, "Unknown instruction: {msg}"),
        }
    }
}
