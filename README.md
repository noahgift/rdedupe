[![Rust CI/CD Pipeline](https://github.com/noahgift/rdedupe/actions/workflows/rust-hello.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rust-hello.yml)

## RDedupe

A Rust based deduplication tool

### Goals

* Build a multiplatform, fast deduplication tool

#### Future Improvements

* Looking into parallelizing walking and hashing i.e. with [jwalk](https://crates.io/crates/jwalk)


### Building and Running

* Build:  cd into rdedupe and run `make all`
* Run:  `cargo run -- dedupe --path tests --pattern .txt`
* Run tests:  `make test`

### OS X Install

* Install rust via [rustup](https://rustup.rs/)
* Add to `~/.cargo/config`

```bash
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```
* run `make all` in rdedupe directory
