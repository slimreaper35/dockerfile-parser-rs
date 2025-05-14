use crate::ast::Instruction;
use crate::parser::utils::process_optional_key_value_pairs;

pub fn parse(arguments: &[String]) -> anyhow::Result<Instruction> {
    let pairs = process_optional_key_value_pairs(arguments);
    Ok(Instruction::Arg(pairs))
}
