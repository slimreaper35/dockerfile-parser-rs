use crate::ast::Instruction;
use crate::parser::options::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options_map, remaining) = get_options_from(arguments);

    let from = options_map.get("from").cloned();
    let chown = options_map.get("chown").cloned();
    let chmod = options_map.get("chmod").cloned();
    let link = options_map.get("link").cloned();

    let mut sources: Vec<String> = remaining;
    if sources.is_empty() {
        anyhow::bail!("Missing source argument for COPY instruction");
    }

    let destination = sources
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Missing destination argument for COPY instruction"))?;

    Ok(Instruction::Copy {
        from,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
