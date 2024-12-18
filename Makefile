all:
	cargo build

run:
	cargo run -- $(ARGS)

test:
	cargo test -- --nocapture

check:
	cargo check

lint:
	cargo clippy -- -D warnings

.PHONY: all run test check lint
