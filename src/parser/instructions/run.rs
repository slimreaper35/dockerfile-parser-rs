use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::utils::clean_exec_form;
use crate::parser::utils::clean_shell_form;
use crate::parser::utils::get_options_from;
use crate::parser::utils::is_exec_form;
use crate::symbols::strings::HEREDOC_START;
use crate::utils::split_heredoc;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.is_empty() {
        return Err(ParseError::MissingArgument(String::from(
            "RUN requires at least one argument",
        )));
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

        return Ok(Instruction::Run {
            mount,
            network,
            security,
            command,
            heredoc,
        });
    }

    let command = if is_exec_form(&remaining) {
        clean_exec_form(&remaining)
    } else {
        clean_shell_form(&remaining)
    };
    let heredoc = None;

    Ok(Instruction::Run {
        mount,
        network,
        security,
        command,
        heredoc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let arguments = vec![
            String::from("--mount=type=bind,source=/host/path,target=/container/path"),
            String::from("--network=host"),
            String::from("--security=seccomp"),
            String::from("cat"),
            String::from("/etc/os-release"),
        ];
        let result = parse(&arguments).unwrap();

        assert_eq!(
            result,
            Instruction::Run {
                mount: Some(String::from(
                    "type=bind,source=/host/path,target=/container/path"
                )),
                network: Some(String::from("host")),
                security: Some(String::from("seccomp")),
                command: vec![String::from("cat"), String::from("/etc/os-release")],
                heredoc: None,
            }
        );
    }
}
