use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let env = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    let (key, value) = env
        .split_once('=')
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    let value = value.trim_start_matches('"').trim_end_matches('"');

    Ok(Instruction::Env {
        key: key.to_string(),
        value: value.to_string(),
    })
}
