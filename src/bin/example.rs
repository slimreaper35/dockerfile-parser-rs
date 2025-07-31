use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;

fn main() {
    let path = PathBuf::from("src/bin/Dockerfile");
    let mut dockerfile = Dockerfile::from(path).unwrap();

    dockerfile.instructions.push(Instruction::Empty);
    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump().unwrap();
    println!("{dockerfile:#?}");
}
