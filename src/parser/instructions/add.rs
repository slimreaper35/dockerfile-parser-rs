use crate::ast::Instruction;
use crate::parser::options::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    let checksum = options.get("checksum").cloned();
    let chown = options.get("chown").cloned();
    let chmod = options.get("chmod").cloned();
    let link = options.get("link").cloned();

    let mut sources: Vec<String> = remaining;
    if sources.is_empty() {
        anyhow::bail!("Missing source argument for ADD instruction");
    }

    let destination = sources
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Missing destination argument for ADD instruction"))?;

    Ok(Instruction::Add {
        checksum,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
