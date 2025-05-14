use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
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

pub fn read_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut current = String::new();
    let mut in_heredoc = false;

    for line in reader.lines().map_while(Result::ok) {
        let trimmed = line.trim_end();

        // skip inline comments
        if trimmed.starts_with(HASHTAG) && !current.is_empty() {
            continue;
        }

        if trimmed.contains(HEREDOC_START) {
            in_heredoc = true;
            current.push_str(trimmed);
            add_heredoc_newline(&mut current);
            continue;
        }

        if in_heredoc {
            current.push_str(trimmed);
            if trimmed == HEREDOC_END {
                lines.push(current);
                current = String::new();
                in_heredoc = false;
            } else {
                add_heredoc_newline(&mut current);
            }
            continue;
        }

        if trimmed.ends_with(BACKSLASH) {
            current.push_str(&trimmed[..trimmed.len() - 1]);
            if !current.ends_with(SPACE) {
                current.push(SPACE);
            }
        } else {
            current.push_str(trimmed);
            lines.push(current);
            current = String::new();
        }
    }
    lines
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

    // double check
    assert!(!instruction.is_empty());
    assert!(!arguments.is_empty());

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
}
