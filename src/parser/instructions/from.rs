use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::options::parse_options;

pub fn parse(args: &str) -> Result<Instruction, ParseError> {
    let (flags_map, remaining) = parse_options(args);

    let platform = flags_map.get("platform").cloned();

    let mut iter = remaining.split_whitespace();
    let image = iter
        .next()
        .ok_or(ParseError::SyntaxError(args.to_string()))?;

    // check if there is an alias
    let keyword = iter.next();
    let alias = iter.next();

    if keyword.is_some() && alias.is_none() {
        return Err(ParseError::SyntaxError(args.to_string()));
    }

    Ok(Instruction::From {
        platform,
        image: image.to_string(),
        alias: alias.map(|a| a.to_string()),
    })
}
