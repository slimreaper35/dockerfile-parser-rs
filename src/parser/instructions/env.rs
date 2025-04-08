use crate::ast::Instruction;
use crate::parser::utils::process_key_value_pairs;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let env = process_key_value_pairs(&arguments);
    Ok(Instruction::Env(env))
}
