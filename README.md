# dockerfile-parser-rs

[![latest version](https://img.shields.io/crates/v/dockerfile-parser-rs?color=orange)](https://crates.io/crates/dockerfile-parser-rs)
[![total downloads](https://img.shields.io/crates/d/dockerfile-parser-rs?color=green)](https://crates.io/crates/dockerfile-parser-rs)
[![license](https://img.shields.io/crates/l/dockerfile-parser-rs?color=yellow)](https://github.com/slimreaper35/dockerfile-parser-rs/blob/main/LICENSE)
[![documentation](https://img.shields.io/docsrs/dockerfile-parser-rs?color=blue)](https://docs.rs/dockerfile-parser-rs/latest/dockerfile_parser_rs)

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
- STOPSIGNAL
- USER
- VOLUME
- WORKDIR

**Note:** In addition to the official
[Dockerfile instructions](https://docs.docker.com/reference/dockerfile/#overview), empty lines and
comments are supported too.

## Usage

### Library

Run the following Cargo command in your project directory:

```shell
cargo add dockerfile-parser-rs
```

Example:

```rust
use std::path::PathBuf;

use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;
use dockerfile_parser_rs::ParseResult;

fn main() -> ParseResult<()> {
    let path = PathBuf::from("./Dockerfile");
    let mut dockerfile = Dockerfile::from(path.clone())?;

    dockerfile.instructions.push(Instruction::User {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump(path)?;
    Ok(())
}
```

### Binary

Run the following Cargo command in your project directory:

```shell
cargo install dockerfile-parser-rs
```

Example:

```shell
# prints the Dockerfile as JSON
dockerfile-parser-rs ./Dockerfile
```

## Limitations

### Instruction case sensitivity

The instructions are not case-sensitive. However, the library works only with uppercase instructions
for simplicity and consistency. Using uppercase instructions is also a recommended convention in
[Dockerfile](https://docs.docker.com/reference/dockerfile/#format) format documentation.

### Instruction arguments ordering

Options for all instructions will be sorted in alphabetical order. This is done to ensure
deterministic output when dumping a Dockerfile. The same applies to the `ARG`, `ENV`, and `LABEL`
instructions when they have multiple key-value pairs defined on one line.

### Here-documents (heredocs)

Here-documents allow redirection of subsequent Dockerfile lines to the input of `RUN` or `COPY`
commands. If such a command contains a here-document, the Dockerfile considers the next lines until
the line only containing a here-doc delimiter as part of the same command.

The here-documents syntax is only supported for the `RUN` instruction and only with the `EOF`
delimiter. Make sure that here-documents are always terminated with an `EOF` character on a new
line.
