use crate::ast::Instruction;
use crate::parser::utils::get_options_from;
use crate::quoter::Quoter;

pub fn parse(arguments: &[String]) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.len() < 2 {
        anyhow::bail!("The ADD instruction must have at least two arguments");
    }

    let checksum = options.get("checksum").cloned();
    let chown = options.get("chown").cloned();
    let chmod = options.get("chmod").cloned();
    let mut link = options.get("link").cloned();

    if link.is_some() && link.clone().unwrap().is_empty() {
        link = Some(String::from("true"));
    }

    let mut sources: Vec<String> = remaining.iter().map(Quoter::dequote).collect();
    let destination = sources.pop().unwrap().dequote();

    Ok(Instruction::Add {
        checksum,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
