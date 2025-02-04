test:
	nice ionice cargo test --workspace --all-targets --all-features

review:
	cargo insta review --all

fmt:
	cargo fmt --all

install:
	cargo install --path . --force
	readlater register

lint: fmt clippy check test

clippy:
	cargo clippy -- -D warnings

check:
	nice cargo check --workspace

build:
	cd webext; pkgx deno run -A build.ts
	cd webext; pkgx npx -y web-ext build --overwrite-dest

webext:
	cd webext; pkgx npx -y web-ext run
