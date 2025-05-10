use crate::ast::Instruction;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::clean_shell_form;
use crate::parser::utils::get_options_from;
use crate::parser::utils::is_exec_form;
use crate::symbols::strings::HEREDOC_START;
use crate::utils::split_heredoc;

pub fn parse(arguments: Vec<String>) -> anyhow::Result<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.is_empty() {
        anyhow::bail!("The RUN instruction must have at least one argument");
    }

    let mount = options.get("mount").cloned();
    let network = options.get("network").cloned();
    let security = options.get("security").cloned();

    if remaining.iter().any(|arg| arg == HEREDOC_START) {
        let lines = split_heredoc(remaining);

        let command = lines.first().unwrap().clone();
        let heredoc = Some(
            lines[1..]
                .to_vec()
                .iter()
                .map(|v| v.join(" "))
                .collect::<Vec<String>>(),
        );

        return Ok(Instruction::RUN {
            mount,
            network,
            security,
            command,
            heredoc,
        });
    }

    let command = if is_exec_form(&remaining) {
        clean_exec_form(remaining)
    } else {
        clean_shell_form(remaining)
    };
    let heredoc = None;

    Ok(Instruction::RUN {
        mount,
        network,
        security,
        command,
        heredoc,
    })
}
