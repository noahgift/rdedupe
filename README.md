[![Tests](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml)
[![Build binary release](https://github.com/noahgift/rdedupe/actions/workflows/release.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/release.yml)
[![Clippy](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml)

## RDedupe

A Rust based deduplication tool

### Goals

* Build a multiplatform, fast deduplication tool that uses Rust parallelization.

#### Current Status

* Added ![Rayon Parallization](https://user-images.githubusercontent.com/58792/209480753-d2452e39-f72b-43c2-8000-b2d9f18d8a33.png)
* Added [progress bar](https://github.com/console-rs/indicatif)
![Progress Bar](https://user-images.githubusercontent.com/58792/209585522-0f12445d-59ca-4e52-8cfd-764a00be6f90.png)




#### Future Improvements

* Add a GUI
* Add a web interface
* Fix GitHub Actions Build process to not fail silently!
* Use Polars DataFrame and include statistics about files and generate a CSV report.
* Store logs about actions performed across multiple runs

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
