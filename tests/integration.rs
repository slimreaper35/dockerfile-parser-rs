use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;

#[test]
fn test_dockerfile_parse() {
    let path = PathBuf::from("tests/data/Dockerfile");
    Dockerfile::from(path).unwrap();
}
