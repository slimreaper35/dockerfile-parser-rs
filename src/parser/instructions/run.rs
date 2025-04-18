use crate::ast::Instruction;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.is_empty() {
        anyhow::bail!("The RUN instruction must have at least one argument");
    }

    let mount = options.get("mount").cloned();
    let network = options.get("network").cloned();
    let security = options.get("security").cloned();

    let command = remaining;

    Ok(Instruction::Run {
        mount,
        network,
        security,
        command,
    })
}
