use crate::ast::Instruction;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let path = arguments
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for WORKDIR instruction"))?;

    Ok(Instruction::Workdir {
        path: path.to_string(),
    })
}
