use crate::ast::Instruction;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    let platform = options.get("platform").cloned();

    let image = remaining
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for FROM instruction"))?;

    // check if there is an alias
    let keyword = remaining.get(1);
    let alias = remaining.get(2);

    if keyword.is_some() && alias.is_none() {
        anyhow::bail!("Missing alias for FROM instruction");
    }

    Ok(Instruction::From {
        platform,
        image: image.to_string(),
        alias: alias.map(|s| s.to_string()),
    })
}
