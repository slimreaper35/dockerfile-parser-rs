use crate::ast::Instruction;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.is_empty() {
        anyhow::bail!("The FROM instruction must have at least one argument");
    }

    let platform = options.get("platform").cloned();
    let image = remaining.first().unwrap().to_owned();
    // check if there is an alias
    let keyword = remaining.get(1);
    let alias = remaining.get(2);

    if keyword.is_some() && alias.is_none() {
        anyhow::bail!("Missing alias for FROM instruction");
    }

    Ok(Instruction::FROM {
        platform,
        image,
        alias: alias.map(String::from),
    })
}
