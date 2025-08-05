.PHONY: fmt fmt-check lint test build doc clean ci

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

clean:
	cargo clean

ci: fmt-check lint test
