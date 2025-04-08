# dockerfile-parser-rs

A pure Rust library for parsing, modifying, and generating Dockerfiles.

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
- USER
- VOLUME
- WORKDIR

**Note:** In addition to the official supported
[Dockerfile instructions](https://docs.docker.com/reference/dockerfile/#overview),
this library also supports placeholders for empty lines and comments.

## Installation

```bash
cargo add dockerfile-parser-rs
```

## Example

```rust
use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;

fn main() {
    let mut dockerfile = Dockerfile::from(PathBuf::from("Dockerfile")).unwrap();

    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump().unwrap();
}
```
