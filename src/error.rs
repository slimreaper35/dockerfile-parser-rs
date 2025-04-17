use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
    #[error("File error: {0}")]
    FileError(String),
}
