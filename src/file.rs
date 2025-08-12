use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::ParseResult;
use crate::ast::Instruction;
use crate::error::ParseError;
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
use crate::parser::instructions::shell;
use crate::parser::instructions::stopsignal;
use crate::parser::instructions::user;
use crate::parser::instructions::volume;
use crate::parser::instructions::workdir;
use crate::symbols::chars::HASHTAG;
use crate::utils::process_dockerfile_content;
use crate::utils::split_instruction_and_arguments;

/// This struct represents a Dockerfile instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dockerfile {
    pub instructions: Vec<Instruction>,
}

impl FromStr for Dockerfile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = process_dockerfile_content(s.lines().map(String::from));

        let instructions = parse(lines)?;
        Ok(Self::new(instructions))
    }
}

impl Dockerfile {
    /// Creates a new `Dockerfile` instance for the given instructions.
    #[must_use]
    pub const fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    /// Creates an empty `Dockerfile` instance.
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Parses the content of the Dockerfile and returns a populated `Dockerfile` instance.
    ///
    /// The file is read line by line, preserving empty lines and comments.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    ///
    /// use dockerfile_parser_rs::Dockerfile;
    /// use dockerfile_parser_rs::ParseResult;
    ///
    /// fn main() -> ParseResult<()> {
    ///     let path = PathBuf::from("./Dockerfile");
    ///
    ///     let dockerfile = Dockerfile::from(path)?;
    ///     dockerfile.to_json();
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an error if the file cannot be opened or if there is a syntax error in the Dockerfile.
    pub fn from(path: PathBuf) -> ParseResult<Self> {
        let file = File::open(path).map_err(|e| ParseError::FileError(e.to_string()))?;
        let reader = BufReader::new(file);
        let lines = process_dockerfile_content(reader.lines().map_while(Result::ok));

        let instructions = parse(lines)?;
        Ok(Self::new(instructions))
    }

    /// Dumps the instructions to a file.
    ///
    /// If the file does not exist, it will be created.
    /// If the file exists, it will be overwritten.
    ///
    /// ## Errors
    ///
    /// Returns an error if the file cannot be created or written to.
    pub fn dump(&self, path: PathBuf) -> ParseResult<()> {
        let mut file = File::create(path).map_err(|e| ParseError::FileError(e.to_string()))?;
        for instruction in &self.instructions {
            writeln!(file, "{instruction}").map_err(|e| ParseError::FileError(e.to_string()))?;
        }
        Ok(())
    }

    /// Writes the Dockerfile to the standard output in JSON format.
    ///
    /// ## Errors
    ///
    /// Returns an error if the Dockerfile cannot be serialized to JSON.
    pub fn to_json(&self) -> ParseResult<()> {
        let json =
            serde_json::to_string_pretty(self).map_err(|e| ParseError::FileError(e.to_string()))?;
        println!("{json}");
        Ok(())
    }

    /// Returns number of instructions in the Dockerfile.
    #[must_use]
    pub fn steps(&self) -> usize {
        self.instructions
            .iter()
            .filter(|i| !matches!(i, Instruction::Empty {} | Instruction::Comment { .. }))
            .count()
    }

    /// Returns number of layers in the Dockerfile.
    #[must_use]
    pub fn layers(&self) -> usize {
        self.instructions
            .iter()
            .filter(|i| {
                matches!(
                    i,
                    Instruction::Add { .. } | Instruction::Copy { .. } | Instruction::Run { .. }
                )
            })
            .count()
    }

    /// Returns number of stages in the Dockerfile.
    #[must_use]
    pub fn stages(&self) -> usize {
        self.instructions
            .iter()
            .filter(|i| matches!(i, Instruction::From { .. }))
            .count()
    }
}

fn parse(lines: Vec<String>) -> ParseResult<Vec<Instruction>> {
    let mut instructions = Vec::new();

    for line in lines {
        // preserve empty lines
        if line.is_empty() {
            instructions.push(Instruction::Empty {});
        // preserve comments
        } else if line.starts_with(HASHTAG) {
            instructions.push(Instruction::Comment(line.clone()));
        } else {
            let (instruction, arguments) = split_instruction_and_arguments(&line)?;
            let instruction = match instruction.as_str() {
                "ADD" => add::parse(&arguments),
                "ARG" => Ok(arg::parse(&arguments)),
                "CMD" => Ok(cmd::parse(&arguments)),
                "COPY" => copy::parse(&arguments),
                "ENTRYPOINT" => Ok(entrypoint::parse(&arguments)),
                "ENV" => Ok(env::parse(&arguments)),
                "EXPOSE" => Ok(expose::parse(arguments)),
                "LABEL" => Ok(label::parse(&arguments)),
                "FROM" => from::parse(&arguments),
                "RUN" => run::parse(&arguments),
                "SHELL" => shell::parse(&arguments),
                "STOPSIGNAL" => stopsignal::parse(&arguments),
                "USER" => user::parse(&arguments),
                "VOLUME" => Ok(volume::parse(&arguments)),
                "WORKDIR" => workdir::parse(&arguments),
                _ => return Err(ParseError::UnknownInstruction(instruction)),
            }?;
            instructions.push(instruction);
        }
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_dummy_dockerfile() -> Dockerfile {
        let instructions = vec![
            Instruction::From {
                platform: None,
                image: String::from("docker.io/library/fedora:latest"),
                alias: Some(String::from("base")),
            },
            Instruction::Run {
                mount: None,
                network: None,
                security: None,
                command: vec![String::from("cat"), String::from("/etc/os-release")],
                heredoc: None,
            },
            Instruction::From {
                platform: None,
                image: String::from("docker.io/library/ubuntu:latest"),
                alias: Some(String::from("builder")),
            },
            Instruction::Copy {
                from: Some(String::from("base")),
                chown: None,
                chmod: None,
                link: None,
                sources: vec![String::from("file.txt")],
                destination: String::from("/tmp/file.txt"),
            },
            Instruction::Entrypoint(vec![String::from("/bin/bash")]),
        ];

        Dockerfile::new(instructions)
    }

    #[test]
    fn test_dockerfile_from_str() {
        let mut content = String::new();
        content.push_str("FROM docker.io/library/fedora:latest\n");
        content.push_str("RUN cat /etc/os-release\n");
        content.push_str("FROM docker.io/library/ubuntu:latest\n");
        content.push_str("COPY file.txt /tmp/file.txt\n");
        content.push_str("ENTRYPOINT [\"/bin/bash\"]\n");

        let dockerfile = Dockerfile::from_str(&content).unwrap();
        assert_eq!(dockerfile.steps(), 5);
        assert_eq!(dockerfile.layers(), 2);
        assert_eq!(dockerfile.stages(), 2);
    }

    #[test]
    fn test_dockerfile_steps() {
        let dockerfile = mock_dummy_dockerfile();
        assert_eq!(dockerfile.steps(), 5);
    }

    #[test]
    fn test_dockerfile_layers() {
        let dockerfile = mock_dummy_dockerfile();
        assert_eq!(dockerfile.layers(), 2);
    }

    #[test]
    fn test_dockerfile_stages() {
        let dockerfile = mock_dummy_dockerfile();
        assert_eq!(dockerfile.stages(), 2);
    }
}
