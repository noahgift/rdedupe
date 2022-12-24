format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-osx:
	cargo build --release --target x86_64-apple-darwin

build-release:
	cargo build --release 

run:
	cargo run -- dedupe --path tests --pattern .txt

all: format lint test run build-release