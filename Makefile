
.PHONY: check, test, fmt-check, fmt-apply, fmt, clippy, build, run

build:
	cargo build

run:
	cargo run

check:
	cargo check

test:
	cargo test

fmt-check:
	cargo fmt --all -- --check

fmt-apply:
	cargo fmt --all --

fmt: fmt-check

clippy:
	cargo clippy -- -D warnings
