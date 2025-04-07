mod ast;
mod error;
mod parser;
mod symbols;
mod utils;

use std::path::Path;

pub use crate::ast::Instruction;
pub use crate::error::ParseError;

use crate::parser::instructions::add;
use crate::parser::instructions::arg;
use crate::parser::instructions::cmd;
use crate::parser::instructions::copy;
use crate::parser::instructions::entrypoint;
use crate::parser::instructions::env;
use crate::parser::instructions::expose;
use crate::parser::instructions::from;
use crate::parser::instructions::label;
use crate::parser::instructions::run;
use crate::parser::instructions::user;
use crate::parser::instructions::volume;
use crate::parser::instructions::workdir;
use crate::utils::read_lines;

use regex::Regex;

const RE: &str = r"^(?P<instruction>[A-Z]+)\s*(?P<args>.*)";

fn tokenize_line(line: &str) -> Result<(&str, &str), ParseError> {
    let regex = Regex::new(RE).unwrap();
    let captures = regex
        .captures(line)
        .ok_or(ParseError::SyntaxError(line.to_string()))?;

    let instruction = captures
        .name("instruction")
        .ok_or(ParseError::SyntaxError(line.to_string()))?
        .as_str();

    let args = captures
        .name("args")
        .ok_or(ParseError::SyntaxError(line.to_string()))?
        .as_str();

    Ok((instruction, args))
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dockerfile {
    pub path: Box<Path>,
    pub instructions: Vec<Instruction>,
}

impl Dockerfile {
    pub fn new(path: Box<Path>) -> Self {
        Dockerfile {
            path,
            instructions: Vec::new(),
        }
    }

    pub fn parse(&self) -> Result<Vec<Instruction>, ParseError> {
        let lines = read_lines(self.path.as_ref());
        let mut instructions = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                // preserve empty lines
                instructions.push(Instruction::Empty);
            } else if trimmed.starts_with("#") {
                // preserve comments
                instructions.push(Instruction::Comment(trimmed.to_string()));
            } else {
                let (keyword, args) = tokenize_line(trimmed)?;
                let instruction = match keyword {
                    "ADD" => add::parse(args),
                    "ARG" => arg::parse(args),
                    "CMD" => cmd::parse(args),
                    "COPY" => copy::parse(args),
                    "ENTRYPOINT" => entrypoint::parse(args),
                    "ENV" => env::parse(args),
                    "EXPOSE" => expose::parse(args),
                    "LABEL" => label::parse(args),
                    "FROM" => from::parse(args),
                    "RUN" => run::parse(args),
                    "USER" => user::parse(args),
                    "VOLUME" => volume::parse(args),
                    "WORKDIR" => workdir::parse(args),
                    _ => return Err(ParseError::UnsupportedInstruction(keyword.to_string())),
                }?;
                instructions.push(instruction);
            }
        }
        Ok(instructions)
    }
}
