use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
use crate::parser::utils::get_options_from;

pub fn parse(arguments: &[String]) -> ParseResult<Instruction> {
    let (options, remaining) = get_options_from(arguments);

    if remaining.is_empty() {
        return Err(ParseError::MissingArgument(String::from(
            "FROM requires either one argument, or three: FROM <source> [AS <name>]",
        )));
    }

    let platform = options.get("platform").cloned();
    let image = remaining.first().unwrap().to_owned();
    // check if there is an alias
    let keyword = remaining.get(1);
    let alias = remaining.get(2);

    if keyword.is_some() && alias.is_none() {
        return Err(ParseError::MissingArgument(String::from(
            "FROM requires either one argument, or three: FROM <source> [AS <name>]",
        )));
    }

    Ok(Instruction::From {
        platform,
        image,
        alias: alias.map(String::from),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_with_alias() {
        let arguments = vec![
            String::from("ubuntu:latest"),
            String::from("AS"),
            String::from("builder"),
        ];
        let result = parse(&arguments).unwrap();

        assert_eq!(
            result,
            Instruction::From {
                platform: None,
                image: String::from("ubuntu:latest"),
                alias: Some(String::from("builder")),
            }
        );
    }

    #[test]
    fn test_from_without_alias() {
        let arguments = vec![String::from("ubuntu:latest")];
        let result = parse(&arguments).unwrap();

        assert_eq!(
            result,
            Instruction::From {
                platform: None,
                image: String::from("ubuntu:latest"),
                alias: None,
            }
        );
    }

    #[test]
    fn test_from_with_invalid_alias() {
        let arguments = vec![String::from("ubuntu:latest"), String::from("AS")];
        let result = parse(&arguments);

        assert!(result.is_err());
    }
}
