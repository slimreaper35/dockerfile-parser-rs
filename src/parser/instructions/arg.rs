use crate::ast::Instruction;
use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::EQUALS;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let name = arguments
        .first()
        .ok_or_else(|| anyhow::anyhow!("Missing argument for ARG instruction"))?;

    if name.contains(EQUALS) && name.split(EQUALS).count() == 2 {
        let (key, value) = name.split_once(EQUALS).unwrap();
        let value = value
            .trim_start_matches(DOUBLE_QUOTE)
            .trim_end_matches(DOUBLE_QUOTE);

        Ok(Instruction::Arg {
            name: key.to_string(),
            default: Some(value.to_string()),
        })
    } else {
        Ok(Instruction::Arg {
            name: name.to_string(),
            default: None,
        })
    }
}
