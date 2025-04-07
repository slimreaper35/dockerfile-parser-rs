use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::options::parse_options;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let (flags_map, remaining) = parse_options(args);

    let mount = flags_map.get("mount").cloned();
    let network = flags_map.get("network").cloned();
    let security = flags_map.get("security").cloned();

    let command = remaining.to_string();
    if command.is_empty() {
        return Err(ParseError::SyntaxError(args.to_string()));
    }

    Ok(Instruction::Run {
        mount,
        network,
        security,
        command,
    })
}
