use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::quoter::Quoter;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    if arguments.len() != 1 {
        return Err(ParseError::BadNumberOfArguments(String::from(
            "WORKDIR requires exactly one argument",
        )));
    }

    let path = arguments.first().unwrap().dequote();
    Ok(Instruction::Workdir { path })
}
