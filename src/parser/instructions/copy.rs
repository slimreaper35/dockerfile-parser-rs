use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::options::parse_options;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let (flags_map, remaining) = parse_options(args);

    let from = flags_map.get("from").cloned();
    let chown = flags_map.get("chown").cloned();
    let chmod = flags_map.get("chmod").cloned();
    let link = flags_map.get("link").cloned();

    let mut sources: Vec<String> = remaining
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let destination = sources
        .pop()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    if sources.is_empty() {
        return Err(ParseError::SyntaxError(args.to_string()));
    }

    Ok(Instruction::Copy {
        from,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
