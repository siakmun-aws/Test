# Rust Hello World

A simple "Hello, World!" program written in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system

## Running the Program

To run the program, use the following command in the root directory of the project:

```bash
cargo run
```

## Building the Program

To build the program without running it:

```bash
cargo build
```

This will create an executable in the `target/debug` directory.

For a release build with optimizations:

```bash
cargo build --release
```

## Testing

This simple program doesn't include tests, but you can run any tests that might be added in the future with:

```bash
cargo test
```