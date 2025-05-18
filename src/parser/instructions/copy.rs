use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::utils::get_options_from;
use crate::quoter::Quoter;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.len() < 2 {
        return Err(ParseError::MissingArgument(String::from(
            "COPY requires at least two arguments",
        )));
    }

    let from = options.get("from").cloned();
    let chown = options.get("chown").cloned();
    let chmod = options.get("chmod").cloned();
    let mut link = options.get("link").cloned();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let arguments = vec![
            String::from("--from=builder"),
            String::from("--chown=root"),
            String::from("--chmod=755"),
            String::from("--link=false"),
            String::from("file.txt"),
            String::from("/tmp/file.txt"),
        ];
        let result = parse(&arguments).unwrap();

        assert_eq!(
            result,
            Instruction::Copy {
                from: Some(String::from("builder")),
                chown: Some(String::from("root")),
                chmod: Some(String::from("755")),
                link: Some(String::from("false")),
                sources: vec![String::from("file.txt")],
                destination: String::from("/tmp/file.txt"),
            }
        );
    }
}
