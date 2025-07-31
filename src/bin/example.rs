use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;
use dockerfile_parser_rs::ParseResult;

fn main() -> ParseResult<()> {
    let path = PathBuf::from("src/bin/Dockerfile");
    let mut dockerfile = Dockerfile::from(path.clone())?;

    dockerfile.instructions.push(Instruction::Empty);
    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump(path)?;
    println!("{dockerfile:#?}");
    Ok(())
}
