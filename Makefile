
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

fmt-save:
	cargo fmt --all --

fmt: fmt-save

clippy-check:
	cargo clippy -- -D warnings

clippy-save:
	cargo clippy --fix --allow-staged -- -D warnings

clippy: clippy-save
