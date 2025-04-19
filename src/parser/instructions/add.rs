use crate::ast::Instruction;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.len() < 2 {
        anyhow::bail!("The ADD instruction must have at least two arguments");
    }

    let checksum = options.get("checksum").cloned();
    let chown = options.get("chown").cloned();
    let chmod = options.get("chmod").cloned();
    let link = options.get("link").cloned();

    let mut sources: Vec<String> = remaining;
    let destination = sources.pop().unwrap();

    Ok(Instruction::Add {
        checksum,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
