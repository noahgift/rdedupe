[![Rust CI/CD Pipeline](https://github.com/noahgift/rdedupe/actions/workflows/rust-hello.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rust-hello.yml)

## RDedupe

A Rust based deduplication tool

### Goals

* Build a multiplatform, fast deduplication tool

### Building and Running

* Build:  cd into rdedupe and run `make all`
* Run:  `cargo run -- dedupe --path tests --pattern .txt`
* Run tests:  `make test`
