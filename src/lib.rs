mod ast;
mod error;
mod parser;
mod symbols;
mod utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// public API
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
use crate::symbols::chars::HASHTAG;
use crate::utils::read_lines;
use crate::utils::tokenize_line;

#[derive(Debug)]
pub struct Dockerfile {
    pub path: PathBuf,
    pub instructions: Vec<Instruction>,
}

impl Dockerfile {
    pub fn new(path: PathBuf) -> Self {
        Dockerfile {
            path,
            instructions: Vec::new(),
        }
    }

    pub fn from(path: PathBuf) -> anyhow::Result<Self> {
        let mut dockerfile = Dockerfile::new(path);
        dockerfile.instructions = dockerfile.parse()?;
        Ok(dockerfile)
    }

    pub fn parse(&self) -> Result<Vec<Instruction>, ParseError> {
        let lines = read_lines(&self.path);
        let mut instructions = Vec::new();

        for line in lines {
            // preserve empty lines
            if line.is_empty() {
                instructions.push(Instruction::Empty);
            // preserve comments
            } else if line.starts_with(HASHTAG) {
                instructions.push(Instruction::Comment(line.to_string()));
            } else {
                let (instruction, arguments) = tokenize_line(&line)?;
                let instruction = match instruction.as_str() {
                    "ADD" => add::parse(arguments),
                    "ARG" => arg::parse(arguments),
                    "CMD" => cmd::parse(arguments),
                    "COPY" => copy::parse(arguments),
                    "ENTRYPOINT" => entrypoint::parse(arguments),
                    "ENV" => env::parse(arguments),
                    "EXPOSE" => expose::parse(arguments),
                    "LABEL" => label::parse(arguments),
                    "FROM" => from::parse(arguments),
                    "RUN" => run::parse(arguments),
                    "USER" => user::parse(arguments),
                    "VOLUME" => volume::parse(arguments),
                    "WORKDIR" => workdir::parse(arguments),
                    _ => return Err(ParseError::UnknownInstruction(instruction)),
                }
                .map_err(|e| ParseError::SyntaxError(e.to_string()))?;
                instructions.push(instruction);
            }
        }
        Ok(instructions)
    }

    pub fn dump(&self) -> anyhow::Result<()> {
        let mut file = File::create(&self.path)?;
        for instruction in &self.instructions {
            writeln!(file, "{}", instruction)?;
        }
        Ok(())
    }
}
