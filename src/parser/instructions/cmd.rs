use crate::ast::Instruction;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::is_exec_form;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let cmd = if is_exec_form(&arguments) {
        clean_exec_form(arguments)
    } else {
        arguments
    };

    Ok(Instruction::Cmd(cmd))
}
