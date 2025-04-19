# dockerfile-parser-rs

[![version](https://img.shields.io/crates/v/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)

The ultimate Rust library for parsing, modifying, and generating Dockerfiles.

## Instructions

The instructions are not case-sensitive. However, the library works only with uppercase instructions
for simplicity and consistency.

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

**Note:** In addition to the official
[Dockerfile instructions](https://docs.docker.com/reference/dockerfile/#overview), this library also
supports placeholders for empty lines and comments.

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

## Limitations

The `RUN`, `CMD`, and `ENTRYPOINT` instructions all have two possible forms:

- `INSTRUCTION ["executable", "param1", "param2"]` (exec form)
- `INSTRUCTION command param1 param2` (shell form)

The exec form makes it possible to avoid shell string munging and to invoke commands using a
specific command shell, or any other executable. It uses a JSON array syntax, where each element in
the array is a command, flag, or argument.

The library parses both forms, but only the exec form is supported for writing instructions back into
a Dockerfile. Internally, these instructions are stored as a vector of strings.

**Note:** The `SHELL` instruction must be written in JSON (exec) form in a Dockerfile.
