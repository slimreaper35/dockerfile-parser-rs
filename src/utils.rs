use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::symbols::chars::BACKSLASH;
use crate::symbols::chars::SPACE;

pub fn read_lines(path: &Path) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e));
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut current = String::new();

    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| panic!("Error reading line: {}", e));
        let trimmed = line.trim();

        if trimmed.ends_with(BACKSLASH) {
            current.push_str(&trimmed[..trimmed.len() - 1]);
            if !current.ends_with(SPACE) {
                current.push(SPACE);
            }
        } else {
            current.push_str(&trimmed);
            lines.push(current);
            current = String::new();
        }
    }

    lines
}
