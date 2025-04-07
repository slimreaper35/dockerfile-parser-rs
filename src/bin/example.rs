use std::path::Path;

use dockerfile_parser::Dockerfile;
use dockerfile_parser::Instruction;
use dockerfile_parser::ParseError;

fn main() -> Result<(), ParseError> {
    let mut dockerfile = Dockerfile::new(Box::from(Path::new("src/bin/Dockerfile")));
    let instructions = dockerfile.parse()?;

    dockerfile.instructions = instructions.clone();
    dockerfile
        .instructions
        .push(Instruction::Comment(String::from(
            "# I have security concerns about this image...",
        )));
    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    println!("{:#?}", dockerfile.instructions);
    Ok(())
}
