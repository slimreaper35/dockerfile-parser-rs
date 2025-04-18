use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
use crate::parser::instructions::user;
use crate::parser::instructions::volume;
use crate::parser::instructions::workdir;
use crate::symbols::chars::HASHTAG;
use crate::utils::read_lines;
use crate::utils::split_instruction_and_arguments;

pub type ParseResult<T> = Result<T, ParseError>;

/// This struct represents a Dockerfile instance.
#[derive(Debug)]
pub struct Dockerfile {
    pub path: PathBuf,
    pub instructions: Vec<Instruction>,
}

impl Dockerfile {
    /// Creates a new empty `Dockerfile` instance for the given path.
    ///
    /// # Notes
    ///
    /// * The actual file does not need to exist at this point.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use dockerfile_parser_rs::Dockerfile;
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let mut dockerfile = Dockerfile::new(PathBuf::from("./Dockerfile"));
    /// dockerfile.instructions.push(Instruction::From {
    ///     platform: None,
    ///     image: String::from("docker.io/library/ubuntu:latest"),
    ///     alias: None,
    /// });
    /// ```
    ///
    pub fn new(path: PathBuf) -> Self {
        Dockerfile {
            path,
            instructions: Vec::new(),
        }
    }

    /// Parses the content of the Dockerfile and returns a populated `Dockerfile` instance.
    ///
    /// # Notes
    ///
    /// * The file is read line by line, preserving empty lines and comments.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] if the file cannot be read or parsed correctly.
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
    ///     println!("{:?}", dockerfile.instructions);
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn from(path: PathBuf) -> ParseResult<Self> {
        let mut dockerfile = Dockerfile::new(path);
        dockerfile.instructions = dockerfile.parse()?;
        Ok(dockerfile)
    }

    /// Parses the content of the Dockerfile and returns a vector of `Instruction` items.
    ///
    /// # Notes
    ///
    /// * The attributes of the `Dockerfile` instance are unmodified.
    /// * The file is read line by line, preserving empty lines and comments.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] if the file cannot be read or parsed correctly.
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
    ///     let dockerfile = Dockerfile::new(PathBuf::from("./Dockerfile"));
    ///     let instructions = dockerfile.parse()?;
    ///     println!("{:?}", instructions);
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn parse(&self) -> ParseResult<Vec<Instruction>> {
        let file = File::open(&self.path).map_err(|e| ParseError::FileError(e.to_string()))?;
        let lines = read_lines(&file);

        let mut instructions = Vec::new();

        for line in lines {
            // preserve empty lines
            if line.is_empty() {
                instructions.push(Instruction::Empty);
            // preserve comments
            } else if line.starts_with(HASHTAG) {
                instructions.push(Instruction::Comment(line.to_owned()));
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

    /// Dumps the current instructions into the Dockerfile.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if writing to the file fails.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use dockerfile_parser_rs::Dockerfile;
    /// use dockerfile_parser_rs::Instruction;
    /// use dockerfile_parser_rs::ParseResult;
    ///
    /// fn main() -> ParseResult<()> {
    ///     let mut dockerfile = Dockerfile::from(PathBuf::from("Dockerfile"))?;
    ///     dockerfile.instructions.push(Instruction::User {
    ///         user: String::from("1001"),
    ///         group: None,
    ///     });
    ///     dockerfile.dump().unwrap();
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn dump(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.path)?;
        for instruction in &self.instructions {
            writeln!(file, "{}", instruction)?;
        }
        Ok(())
    }
}
