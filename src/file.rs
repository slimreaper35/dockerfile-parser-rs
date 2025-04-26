use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
use crate::utils::read_lines;
use crate::utils::split_instruction_and_arguments;

/// This struct represents a Dockerfile instance.
#[derive(Debug)]
pub struct Dockerfile {
    pub path: PathBuf,
    pub instructions: Vec<Instruction>,
}

impl Dockerfile {
    /// Creates a new `Dockerfile` instance for the given path and instructions.
    ///
    /// The actual file does not need to exist at this point.
    pub fn new(path: PathBuf, instructions: Vec<Instruction>) -> Self {
        Dockerfile { path, instructions }
    }

    /// Creates an empty `Dockerfile` instance for the given path.
    ///
    /// The actual file does not need to exist at this point.
    pub fn empty(path: PathBuf) -> Self {
        Dockerfile::new(path, Vec::new())
    }

    /// Parses the content of the Dockerfile and returns a populated `Dockerfile` instance.
    ///
    /// The file is read line by line, preserving empty lines and comments.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use dockerfile_parser_rs::Dockerfile;
    /// use dockerfile_parser_rs::ParseResult;
    ///
    /// fn main() -> ParseResult<()> {
    ///     let dockerfile = Dockerfile::from(PathBuf::from("./Dockerfile"))?;
    ///     println!("{:#?}", dockerfile.instructions);
    ///     Ok(())
    /// }
    /// ```
    pub fn from(path: PathBuf) -> ParseResult<Self> {
        let mut dockerfile = Dockerfile::empty(path);
        dockerfile.instructions = dockerfile.parse()?;
        Ok(dockerfile)
    }

    /// Parses the content of the Dockerfile and returns a vector of `Instruction` items.
    ///
    /// The file is read line by line, preserving empty lines and comments.
    ///
    /// **The attributes of the `Dockerfile` instance are not modified by this method.**
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use dockerfile_parser_rs::Dockerfile;
    /// use dockerfile_parser_rs::ParseResult;
    ///
    /// fn main() -> ParseResult<()> {
    ///     let dockerfile = Dockerfile::empty(PathBuf::from("./Dockerfile"));
    ///     let instructions = dockerfile.parse()?;
    ///     println!("{:#?}", instructions);
    ///     Ok(())
    /// }
    /// ```
    pub fn parse(&self) -> ParseResult<Vec<Instruction>> {
        let file = File::open(&self.path).map_err(|e| ParseError::FileError(e.to_string()))?;
        let lines = read_lines(&file);

        let mut instructions = Vec::new();

        for line in lines {
            // preserve empty lines
            if line.is_empty() {
                instructions.push(Instruction::EMPTY);
            // preserve comments
            } else if line.starts_with(HASHTAG) {
                instructions.push(Instruction::COMMENT(line.to_owned()));
            } else {
                let (instruction, arguments) = split_instruction_and_arguments(&line)?;
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
                    "SHELL" => shell::parse(arguments),
                    "STOPSIGNAL" => stopsignal::parse(arguments),
                    "USER" => user::parse(arguments),
                    "VOLUME" => volume::parse(arguments),
                    "WORKDIR" => workdir::parse(arguments),
                    _ => return Err(ParseError::UnknownInstruction(instruction)),
                };
                match instruction {
                    Ok(instruction) => instructions.push(instruction),
                    Err(e) => {
                        return Err(ParseError::SyntaxError(format!("{}: {}", line, e)));
                    }
                }
            }
        }
        Ok(instructions)
    }

    /// Dumps the current instructions into the Dockerfile.
    ///
    /// If the file does not exist, it will be created.
    /// If the file exists, it will be overwritten.
    pub fn dump(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.path)?;
        for instruction in &self.instructions {
            writeln!(file, "{}", instruction)?;
        }
        Ok(())
    }
}
