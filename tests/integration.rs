use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;

#[test]
fn test_dockerfile_parse() -> anyhow::Result<()> {
    Dockerfile::from(PathBuf::from("tests/Dockerfile"))?;
    Ok(())
}
