use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid instruction syntax: {0}")]
    SyntaxError(String),
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
}
