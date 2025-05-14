use crate::ast::Instruction;

pub fn parse(arguments: Vec<String>) -> Instruction {
    let ports = arguments;
    Instruction::Expose { ports }
}
