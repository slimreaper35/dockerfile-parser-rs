use std::str::FromStr;

use crate::ast::Instruction;
use crate::ast::Protocol;
use crate::symbols::chars::SLASH;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let port = arguments
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for EXPOSE instruction"))?;

    // check if there is a protocol
    let (port, protocol) = match port.split_once(SLASH) {
        Some((port, protocol)) => (port.to_string(), Some(Protocol::from_str(protocol)?)),
        None => (port.to_string(), None),
    };

    Ok(Instruction::Expose { port, protocol })
}
