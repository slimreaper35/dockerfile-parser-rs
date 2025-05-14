use crate::ast::Instruction;
use crate::parser::utils::process_key_value_pairs;

pub fn parse(arguments: &[String]) -> Instruction {
    let pairs = process_key_value_pairs(arguments);
    Instruction::Env(pairs)
}
