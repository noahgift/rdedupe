format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-release:
	cargo build --release 

run:
	cargo run -- dedupe --path tests --pattern .txt

all: format lint test run build-release