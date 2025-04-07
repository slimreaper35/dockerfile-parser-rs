use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid instruction syntax: {0}")]
    SyntaxError(String),
    #[error("Unsupported instruction: {0}")]
    UnsupportedInstruction(String),
}
