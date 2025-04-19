use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
}
