use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use crate::error::ParseError;
use crate::symbols::chars::BACKSLASH;
use crate::symbols::chars::HASHTAG;
use crate::symbols::chars::SPACE;

pub fn read_lines(path: &PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e));
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut current = String::new();

    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| panic!("Error reading line: {}", e));
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

pub fn tokenize_line(line: &str) -> Result<(String, Vec<String>), ParseError> {
    // https://docs.docker.com/reference/dockerfile/#format
    let regex = regex::Regex::new(r"^(?P<instruction>[A-Z]+)\s*(?P<arguments>.*)").unwrap();

    let captures = regex
        .captures(line)
        .ok_or_else(|| ParseError::SyntaxError(line.to_string()))?;

    let instruction = captures
        .name("instruction")
        .ok_or_else(|| ParseError::SyntaxError(line.to_string()))?
        .as_str();

    let arguments = captures
        .name("arguments")
        .ok_or_else(|| ParseError::SyntaxError(line.to_string()))?
        .as_str();

    Ok((
        instruction.to_string(),
        arguments
            .split_whitespace()
            .map(|s| s.to_string())
            .collect(),
    ))
}
