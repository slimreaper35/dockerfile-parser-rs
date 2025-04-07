use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let mount = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    let mount = mount.trim_start_matches("[\"").trim_end_matches("\"]");

    Ok(Instruction::Volume {
        mount: mount.to_string(),
    })
}
