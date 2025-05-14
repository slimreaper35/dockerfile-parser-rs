use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::is_exec_form;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    if !is_exec_form(arguments) {
        return Err(ParseError::SyntaxError(String::from(
            "SHELL requires the arguments to be in JSON form",
        )));
    }

    let shell = clean_exec_form(arguments);
    Ok(Instruction::Shell(shell))
}
