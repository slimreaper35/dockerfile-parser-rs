# dockerfile-parser-rs

[![version](https://img.shields.io/crates/v/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)

The ultimate Rust library for parsing, modifying, and generating Dockerfiles.

## Instructions

- ADD
- ARG
- CMD
- COPY
- ENTRYPOINT
- ENV
- EXPOSE
- FROM
- LABEL
- RUN
- SHELL
- USER
- VOLUME
- WORKDIR

**Note:** In addition to the official supported
[Dockerfile instructions](https://docs.docker.com/reference/dockerfile/#overview),
this library also supports placeholders for empty lines and comments.

## Install

Run the following Cargo command in your project directory:

```bash
cargo add dockerfile-parser-rs
```

## Example

```rust
use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;

fn main() {
    let path = PathBuf::from("Dockerfile");
    let mut dockerfile = Dockerfile::from(path).unwrap();

    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump().unwrap();
}
```
