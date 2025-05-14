use crate::ast::Instruction;
use crate::parser::utils::process_optional_key_value_pairs;

pub fn parse(arguments: &[String]) -> Instruction {
    let pairs = process_optional_key_value_pairs(arguments);
    Instruction::Arg(pairs)
}
