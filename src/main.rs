use std::fs;
use std::path::PathBuf;

use clap::Parser;
use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::ParseError;
use dockerfile_parser_rs::ParseResult;

/// A parser that prints Dockerfiles as JSON
#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(help = "Path to the Dockerfile")]
    path: PathBuf,
    #[arg(short, long, help = "Path to the output file")]
    output: Option<PathBuf>,
}

fn main() -> ParseResult<()> {
    let args = Args::parse();
    let dockerfile = Dockerfile::from(args.path)?;
    let json = dockerfile.to_json()?;

    if let Some(output) = args.output {
        fs::write(output, json).map_err(|e| ParseError::FileError(e.to_string()))?;
    } else {
        println!("{json}");
    }

    Ok(())
}
