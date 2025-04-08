use crate::ast::Instruction;
use crate::parser::utils::is_exec_form;
use crate::symbols::chars::COMMA;
use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::LEFT_BRACKET;
use crate::symbols::chars::RIGHT_BRACKET;
use crate::symbols::strings::EMPTY;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let mut entrypoint: Vec<String> = Vec::new();

    if is_exec_form(&arguments) {
        for arg in arguments {
            let entry = arg
                .trim_start_matches(LEFT_BRACKET)
                .trim_end_matches(RIGHT_BRACKET)
                .replace([DOUBLE_QUOTE, COMMA], EMPTY);

            if !entry.is_empty() {
                entrypoint.push(entry);
            }
        }
    } else {
        entrypoint.extend(arguments);
    };

    Ok(Instruction::Entrypoint(entrypoint))
}
