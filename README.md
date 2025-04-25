# dockerfile-parser-rs

[![version](https://img.shields.io/crates/v/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)

The ultimate Rust library for parsing, modifying, and generating Dockerfiles.

**If you find it useful, please consider giving it a star on GitHub. Thank you so much!**

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

    dockerfile.instructions.push(Instruction::USER {
        user: String::from("1001"),
        group: None,
    });

    dockerfile.dump().unwrap();
}
```

## Limitations

...or things to keep in mind when using this library.

### Shell and exec form

The `RUN`, `CMD`, and `ENTRYPOINT` instructions all have two possible forms:

- `INSTRUCTION ["executable", "param1", "param2"]` (exec form)
- `INSTRUCTION command param1 param2` (shell form)

The exec form makes it possible to avoid shell string munging and to invoke commands using a
specific command shell, or any other executable. It uses a JSON array syntax, where each element in
the array is a command, flag, or argument.

The library parses both forms, but only the exec form is supported for writing instructions back
into a Dockerfile.

**Note:** The `SHELL` instruction must be written in JSON (exec) form in a Dockerfile.

### Instruction arguments ordering

Options for all instructions will be sorted in alphabetical order. This is done to ensure
deterministic output when dumping a Dockerfile. The same applies to the `ARG`, `ENV`, and `LABEL`
instructions when they have multiple key-value pairs defined on one line.

### Instruction case sensitivity

The instructions are not case-sensitive. However, the library works only with uppercase instructions
for simplicity and consistency. Using uppercase instructions is also recommended convention in
[Dockerfile](https://docs.docker.com/reference/dockerfile/#format) format documentation.
