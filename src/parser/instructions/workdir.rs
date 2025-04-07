use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let path = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    Ok(Instruction::Workdir {
        path: path.to_string(),
    })
}
