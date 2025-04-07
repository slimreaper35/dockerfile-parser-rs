use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let mut iter = args.split_whitespace();
    let arg = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    if arg.contains('=') && arg.split('=').count() == 2 {
        let (key, value) = arg.split_once('=').unwrap();
        let value = value.trim_start_matches('"').trim_end_matches('"');

        Ok(Instruction::Arg {
            name: key.to_string(),
            default: Some(value.to_string()),
        })
    } else {
        Ok(Instruction::Arg {
            name: arg.to_string(),
            default: None,
        })
    }
}
