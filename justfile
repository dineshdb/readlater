test:
	nice ionice cargo test --workspace --all-targets --all-features

review:
	cargo insta review --all

fmt:
	cargo fmt --all

install:
	cargo install --path . --force

lint: fmt clippy check test

clippy:
	cargo clippy -- -D warnings

check:
	nice cargo check --workspace
