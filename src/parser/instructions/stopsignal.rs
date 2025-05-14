use crate::ast::Instruction;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    if arguments.len() != 1 {
        anyhow::bail!("The STOPSIGNAL instruction must have exactly one argument");
    }

    let signal = arguments.first().unwrap().to_owned();
    Ok(Instruction::Stopsignal { signal })
}
