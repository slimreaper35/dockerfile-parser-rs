use crate::ast::Instruction;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::is_exec_form;

pub fn parse(arguments: &[String]) -> anyhow::Result<Instruction> {
    if !is_exec_form(arguments) {
        anyhow::bail!("The SHELL instruction must be written in JSON form");
    }

    // must be in exec form
    let shell = clean_exec_form(arguments);
    Ok(Instruction::Shell(shell))
}
