use std::sync::LazyLock;

use regex::Regex;

use crate::ParseResult;
use crate::error::ParseError;
use crate::symbols::chars::BACKSLASH;
use crate::symbols::chars::HASHTAG;
use crate::symbols::chars::SPACE;
use crate::symbols::strings::HEREDOC_END;
use crate::symbols::strings::HEREDOC_NEWLINE;
use crate::symbols::strings::HEREDOC_START;

pub fn process_dockerfile_content<I>(lines_iter: I) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    let mut result = Vec::new();

    let mut current_line = String::new();
    let mut in_heredoc = false;

    for line in lines_iter {
        let trimmed_line = line.trim();

        // skip inline comments
        if trimmed_line.starts_with(HASHTAG) && !current_line.is_empty() {
            continue;
        }

        if trimmed_line.contains(HEREDOC_START) {
            in_heredoc = true;
            current_line.push_str(trimmed_line);
            add_heredoc_newline(&mut current_line);
            continue;
        }

        if in_heredoc && trimmed_line == HEREDOC_END {
            current_line.push_str(HEREDOC_END);
            result.push(current_line);
            current_line = String::new();
            in_heredoc = false;
            continue;
        }

        if in_heredoc {
            current_line.push_str(trimmed_line);
            add_heredoc_newline(&mut current_line);
            continue;
        }

        if trimmed_line.ends_with(BACKSLASH) {
            current_line.push_str(&trimmed_line[..trimmed_line.len() - 1]);
            current_line.push(SPACE);
        } else {
            current_line.push_str(trimmed_line);
            result.push(current_line);
            current_line = String::new();
        }
    }
    result
}

pub fn split_heredoc(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current_sub_vector: Vec<String> = Vec::new();

    for s in strings {
        if s == HEREDOC_NEWLINE {
            if !current_sub_vector.is_empty() {
                result.push(current_sub_vector);
            }
            current_sub_vector = Vec::new();
        } else {
            current_sub_vector.push(s);
        }
    }

    if !current_sub_vector.is_empty() {
        result.push(current_sub_vector);
    }

    result
}

fn add_heredoc_newline(string: &mut String) {
    string.push(SPACE);
    string.push_str(HEREDOC_NEWLINE);
    string.push(SPACE);
}

pub fn split_instruction_and_arguments(line: &str) -> ParseResult<(String, Vec<String>)> {
    // https://docs.docker.com/reference/dockerfile/#format
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(?P<instruction>[A-Z][A-Z0-9]*)\s+(?P<arguments>\S.+)$").unwrap()
    });

    let captures = RE
        .captures(line)
        .ok_or_else(|| ParseError::SyntaxError(line.to_owned()))?;

    let instruction = captures
        .name("instruction")
        .ok_or_else(|| ParseError::SyntaxError(line.to_owned()))?
        .as_str();

    let arguments = captures
        .name("arguments")
        .ok_or_else(|| ParseError::SyntaxError(line.to_owned()))?
        .as_str();

    Ok((
        instruction.to_owned(),
        // preserve tabs inside heredocs
        arguments
            .split(SPACE)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_empty_line_fails() {
        let line = "";
        let result = split_instruction_and_arguments(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_lowercase_instruction_fails() {
        let line = "run arg1 arg2";
        let result = split_instruction_and_arguments(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_instruction_and_missing_arguments_fails() {
        let line = "RUN";
        let result = split_instruction_and_arguments(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_instruction_and_arguments() {
        let line = "RUN arg1 arg2";
        let result = split_instruction_and_arguments(line);
        assert!(result.is_ok());

        let (instruction, arguments) = result.unwrap();
        assert_eq!(instruction, "RUN");
        assert_eq!(arguments, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_add_heredoc_newline() {
        let mut string = String::from("test");
        add_heredoc_newline(&mut string);
        assert_eq!(string, format!("test {} ", HEREDOC_NEWLINE));
    }

    #[test]
    fn test_split_heredoc() {
        let strings = vec![
            String::from("test1"),
            String::from(HEREDOC_NEWLINE),
            String::from("test2"),
            String::from(HEREDOC_NEWLINE),
            String::from("test3"),
        ];
        let result = split_heredoc(strings);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec![String::from("test1")]);
        assert_eq!(result[1], vec![String::from("test2")]);
        assert_eq!(result[2], vec![String::from("test3")]);
    }
}
