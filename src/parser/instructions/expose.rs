use std::str::FromStr;

use crate::Protocol;
use crate::ast::Instruction;
use crate::symbols::chars::SLASH;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    if arguments.len() != 1 {
        anyhow::bail!("The EXPOSE instruction must have exactly one argument");
    }

    let port = arguments.first().unwrap();
    // check if there is a protocol
    let (port, protocol) = match port.split_once(SLASH) {
        Some((port, protocol)) => (port.to_owned(), Some(parse_protocol(protocol)?)),
        None => (port.to_owned(), None),
    };

    Ok(Instruction::Expose { port, protocol })
}

fn parse_protocol(s: &str) -> anyhow::Result<Protocol> {
    Protocol::from_str(s).map_err(|_| anyhow::anyhow!("Invalid protocol for EXPOSE instruction"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invalid_protocol_fails() {
        let result = parse_protocol("http");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_valid_uppercase_protocols_fails() {
        let result = parse_protocol("TCP");
        assert!(result.is_err());

        let result = parse_protocol("UDP");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_valid_protocols() {
        let result = parse_protocol("tcp");
        assert!(result.is_ok());

        let result = parse_protocol("udp");
        assert!(result.is_ok());
    }
}
