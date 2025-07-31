use std::io::Write;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::ParseError;

#[test]
fn test_parse() {
    let path = std::path::PathBuf::from("tests/data/Dockerfile.example");
    Dockerfile::from(path).unwrap();
}

#[test]
fn test_parse_and_dump() {
    let path = std::path::PathBuf::from("tests/data/Dockerfile.complex");
    let dockerfile = Dockerfile::from(path.clone()).unwrap();

    let temp_file = std::env::temp_dir().join("Dockerfile.complex");
    dockerfile.dump(temp_file.clone()).unwrap();

    let original_content = std::fs::read_to_string(path).unwrap();
    let dumped_content = std::fs::read_to_string(temp_file).unwrap();
    assert_eq!(original_content, dumped_content);
}
#[test]
fn test_invalid_instruction_name() {
    let temp_file = std::env::temp_dir().join("Dockerfile.temp");

    let mut file = std::fs::File::create(temp_file.clone()).unwrap();
    writeln!(file, "MAKE love").unwrap();

    let result = Dockerfile::from(temp_file);
    assert!(matches!(result, Err(ParseError::UnknownInstruction(_))));
}
