use crate::ast::Instruction;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options_map, remaining) = get_options_from(arguments);

    if remaining.len() < 2 {
        anyhow::bail!("The COPY instruction must have at least two arguments");
    }

    let from = options_map.get("from").cloned();
    let chown = options_map.get("chown").cloned();
    let chmod = options_map.get("chmod").cloned();
    let link = options_map.get("link").cloned();

    let mut sources: Vec<String> = remaining;
    let destination = sources.pop().unwrap();

    Ok(Instruction::Copy {
        from,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
