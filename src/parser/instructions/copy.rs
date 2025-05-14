use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::utils::get_options_from;
use crate::quoter::Quoter;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    let (options_map, remaining) = get_options_from(arguments);

    if remaining.len() < 2 {
        return Err(ParseError::MissingArgument(String::from(
            "COPY requires at least two arguments",
        )));
    }

    let from = options_map.get("from").cloned();
    let chown = options_map.get("chown").cloned();
    let chmod = options_map.get("chmod").cloned();
    let mut link = options_map.get("link").cloned();

    if link.is_some() && link.clone().unwrap().is_empty() {
        link = Some(String::from("true"));
    }

    let mut sources: Vec<String> = remaining.iter().map(Quoter::dequote).collect();
    let destination = sources.pop().unwrap().dequote();

    Ok(Instruction::Copy {
        from,
        chown,
        chmod,
        link,
        sources,
        destination,
    })
}
