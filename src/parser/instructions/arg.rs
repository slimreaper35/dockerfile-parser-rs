use crate::ast::Instruction;
use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::EQUALS;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    if arguments.len() != 1 {
        anyhow::bail!("The ARG instruction must have exactly one argument");
    }

    let name = arguments.first().unwrap().to_owned();
    // check if there is a default value
    if let Some((key, value)) = name.split_once(EQUALS) {
        let value = value
            .trim_start_matches(DOUBLE_QUOTE)
            .trim_end_matches(DOUBLE_QUOTE);

        Ok(Instruction::Arg {
            name: key.to_owned(),
            default: Some(value.to_owned()),
        })
    } else {
        Ok(Instruction::Arg {
            name,
            default: None,
        })
    }
}
