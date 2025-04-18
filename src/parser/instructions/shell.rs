use crate::ast::Instruction;
use crate::parser::utils::is_exec_form;
use crate::symbols::chars::COMMA;
use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::LEFT_BRACKET;
use crate::symbols::chars::RIGHT_BRACKET;
use crate::symbols::strings::EMPTY;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let mut shell: Vec<String> = Vec::new();

    // must be in exec form
    if !is_exec_form(&arguments) {
        anyhow::bail!("Invalid SHELL instruction");
    }

    for arg in arguments {
        let arg = arg
            .trim_start_matches(LEFT_BRACKET)
            .trim_end_matches(RIGHT_BRACKET)
            .replace([DOUBLE_QUOTE, COMMA], EMPTY);

        if !arg.is_empty() {
            shell.push(arg);
        }
    }

    Ok(Instruction::Shell(shell))
}
