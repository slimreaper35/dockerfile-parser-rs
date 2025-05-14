use crate::ast::Instruction;
use crate::quoter::Quoter;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    if arguments.len() != 1 {
        anyhow::bail!("The WORKDIR instruction must have exactly one argument");
    }

    let path = arguments.first().unwrap().dequote();
    Ok(Instruction::Workdir { path })
}
