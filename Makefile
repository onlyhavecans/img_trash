all: lint test
.PHONY: all

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
.PHONY: lint

test:
	cargo test
.PHONY: test

clean:
	cargo clean
.PHONY: test
