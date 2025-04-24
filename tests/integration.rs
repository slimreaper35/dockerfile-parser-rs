use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;

#[test]
fn test_dockerfile_parse() {
    let path = PathBuf::from("tests/data/Dockerfile.simple");
    Dockerfile::from(path).unwrap();
}

#[test]
fn test_dockerfile_parse_and_dump() {
    let path = PathBuf::from("tests/data/Dockerfile.complex");
    let mut dockerfile = Dockerfile::from(path).unwrap();

    dockerfile.path = PathBuf::from("tests/data/Dockerfile.tmp");
    dockerfile.dump().unwrap();

    let original_content =
        std::fs::read_to_string(PathBuf::from("tests/data/Dockerfile.complex")).unwrap();
    let dumped_content =
        std::fs::read_to_string(PathBuf::from("tests/data/Dockerfile.tmp")).unwrap();

    std::fs::remove_file(PathBuf::from("tests/data/Dockerfile.tmp")).unwrap();
    assert_eq!(original_content, dumped_content);
}
