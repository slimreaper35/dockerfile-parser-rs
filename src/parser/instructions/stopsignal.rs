use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    if arguments.len() != 1 {
        return Err(ParseError::BadNumberOfArguments(String::from(
            "STOPSIGNAL requires exactly one argument",
        )));
    }

    let signal = arguments.first().unwrap().to_owned();
    Ok(Instruction::Stopsignal { signal })
}
