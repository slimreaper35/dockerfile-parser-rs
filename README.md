# dockerfile-parser-rs

[![latest version](https://img.shields.io/crates/v/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)
[![total downloads](https://img.shields.io/crates/d/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)

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

## Install

Run the following Cargo command in your project directory:

```shell
cargo add dockerfile-parser-rs
```

## Example

Check out the [src/bin/example.rs](src/bin/example.rs) file or run the following commands to see how
to use the library.

```shell
git clone https://github.com/slimreaper35/dockerfile-parser-rs.git
cd dockerfile-parser-rs
cargo run example
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
