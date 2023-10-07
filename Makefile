.PHONY: default

default:
	cargo test --features globals

lint:
	cargo check --features globals
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings
