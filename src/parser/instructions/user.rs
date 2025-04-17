use crate::ast::Instruction;
use crate::symbols::chars::COLON;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let user = arguments
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for USER instruction"))?;

    // check if there is a group
    let (user, group) = match user.split_once(COLON) {
        Some((user, group)) => (user.to_owned(), Some(group.to_owned())),
        None => (user.to_owned(), None),
    };

    Ok(Instruction::User { user, group })
}
