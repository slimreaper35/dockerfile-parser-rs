# dockerfile-parser-rs

[![version](https://img.shields.io/crates/v/dockerfile-parser-rs)](https://crates.io/crates/dockerfile-parser-rs)

The ultimate Rust library for parsing, modifying, and generating Dockerfiles with
**only one dependency**. It provides a simple and efficient way to work with Dockerfiles in Rust.

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

Check out the [src/bin/example.rs](src/bin/example.rs) file and run the following command:

```shell
cargo run example
```

## Limitations

...or things to keep in mind when using this library.

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

### Shell and exec form

The `RUN`, `CMD`, and `ENTRYPOINT` instructions all have two possible forms:

- `INSTRUCTION ["executable", "param1", "param2"]` (exec form)
- `INSTRUCTION command param1 param2` (shell form)

The exec form makes it possible to avoid shell string munging and to invoke commands using a
specific command shell, or any other executable. It uses a JSON array syntax, where each element in
the array is a command, flag, or argument.

The library parses both forms, but only the exec form is supported for writing instructions back
into a Dockerfile.
