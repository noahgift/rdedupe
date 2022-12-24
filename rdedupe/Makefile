format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-osx:
	#TBD
	#rustup target add x86_64-apple-darwin
	#cargo build --release --target x86_64-apple-darwin

run:
	cargo run -- dedupe --path tests --pattern .txt

all: format lint test run