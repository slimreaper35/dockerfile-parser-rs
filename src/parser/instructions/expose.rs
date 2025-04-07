use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let port = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    match port.parse() {
        Ok(port) => Ok(Instruction::Expose { port }),
        Err(_) => Err(ParseError::SyntaxError(args.to_string())),
    }
}
