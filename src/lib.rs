mod ast;
mod error;
mod file;
mod parser;
mod symbols;
mod utils;

// public API

pub type ParseResult<T> = Result<T, ParseError>;

pub use crate::ast::Instruction;
pub use crate::ast::Protocol;

pub use crate::error::ParseError;

pub use crate::file::Dockerfile;
