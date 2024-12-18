FILENAME   ?= ./examples/rogue.obj

run:
	cargo run $(FILENAME)

test:
	cargo test -- --nocapture

check:
	cargo check

lint:
	cargo clippy -- -D warnings

.PHONY: run test check lint

