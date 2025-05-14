use crate::ast::Instruction;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let ports = arguments;
    Ok(Instruction::Expose { ports })
}
