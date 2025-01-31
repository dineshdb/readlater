test:
	cargo test --workspace --all-targets --all-features

review:
	cargo insta review --all

fmt:
	cargo fmt --all
