use crate::ast::Instruction;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::is_exec_form;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    // must be in exec form
    if !is_exec_form(&arguments) {
        anyhow::bail!("The SHELL instruction must be written in JSON form in a Dockerfile");
    }

    let shell = clean_exec_form(arguments);
    Ok(Instruction::Shell(shell))
}
