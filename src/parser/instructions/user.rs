use crate::ast::Instruction;
use crate::error::ParseError;

const COLON: char = ':';

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let user_with_group = iter
        .next()
        .ok_or_else(|| ParseError::SyntaxError(args.to_string()))?;

    // check if there is a group
    let mut parts = user_with_group.splitn(2, COLON);
    let user = parts.next().unwrap().to_string();
    let group = parts.next().map(|g| g.to_string());

    Ok(Instruction::User { user, group })
}
