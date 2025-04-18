use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::ParseResult;
use crate::error::ParseError;
use crate::symbols::chars::BACKSLASH;
use crate::symbols::chars::HASHTAG;
use crate::symbols::chars::SPACE;

pub fn read_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut current = String::new();

    for line in reader.lines().map_while(Result::ok) {
        let trimmed = line.trim();

        // skip inline comments
        if trimmed.starts_with(HASHTAG) && !current.is_empty() {
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

pub fn split_instruction_and_arguments(line: &str) -> ParseResult<(String, Vec<String>)> {
    // https://docs.docker.com/reference/dockerfile/#format
    static RE: Lazy<Regex> = Lazy::new(|| {
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
        arguments.split_whitespace().map(String::from).collect(),
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
