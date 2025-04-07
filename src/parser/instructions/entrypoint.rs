use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let command = args.trim().to_string();
    Ok(Instruction::Entrypoint { command })
}
