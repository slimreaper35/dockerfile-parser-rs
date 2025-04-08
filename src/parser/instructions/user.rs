use crate::ast::Instruction;
use crate::symbols::chars::COLON;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let user = arguments
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for USER instruction"))?;

    // check if there is a group
    let (user, group) = match user.split_once(COLON) {
        Some((user, group)) => (user.to_string(), Some(group.to_string())),
        None => (user.to_string(), None),
    };

    Ok(Instruction::User { user, group })
}
