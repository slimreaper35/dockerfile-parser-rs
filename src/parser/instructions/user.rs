use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::quoter::Quoter;
use crate::symbols::chars::COLON;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    if arguments.len() != 1 {
        return Err(ParseError::BadNumberOfArguments(String::from(
            "USER requires exactly one argument",
        )));
    }

    let user = arguments.first().unwrap().dequote();
    // check if there is a group
    let (user, group) = match user.split_once(COLON) {
        Some((user, group)) => (user.to_owned(), Some(group.to_owned())),
        None => (user, None),
    };

    Ok(Instruction::User { user, group })
}
