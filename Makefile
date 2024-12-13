all:
	cargo build

run:
	cargo run

test:
	cargo test -- --nocapture

check:
	cargo check

lint:
	cargo clippy -- -D warnings

.PHONY: all run test check lint
