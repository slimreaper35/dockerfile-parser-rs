use std::path::PathBuf;

use clap::Parser;
use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::ParseResult;

/// A parser that prints Dockerfiles as JSON
#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(help = "Path to the Dockerfile")]
    path: PathBuf,
}

fn main() -> ParseResult<()> {
    let args = Args::parse();
    let dockerfile = Dockerfile::from(args.path)?;

    dockerfile.to_json()?;
    Ok(())
}
