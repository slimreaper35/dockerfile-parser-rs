use crate::ast::Instruction;

pub const fn parse(arguments: Vec<String>) -> Instruction {
    let ports = arguments;
    Instruction::Expose { ports }
}
