use crate::ast::Instruction;
use crate::quoter::Quoter;
use crate::symbols::chars::COLON;

pub fn parse(arguments: &[String]) -> anyhow::Result<Instruction> {
    if arguments.len() != 1 {
        anyhow::bail!("The USER instruction must have exactly one argument");
    }

    let user = arguments.first().unwrap().dequote();
    // check if there is a group
    let (user, group) = match user.split_once(COLON) {
        Some((user, group)) => (user.to_owned(), Some(group.to_owned())),
        None => (user, None),
    };

    Ok(Instruction::User { user, group })
}
