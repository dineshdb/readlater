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
	cd webext; pkgx npx -y web-ext build --overwrite-dest

webext:
	ffmpeg -i webext/icons/icon.svg -y -vf scale=48:48 webext/icons/icon-48.png
	ffmpeg -i webext/icons/icon.svg -y -vf scale=38:38 webext/icons/icon-38.png
	ffmpeg -i webext/icons/icon.svg -y -vf scale=24:24 webext/icons/icon-24.png
	ffmpeg -i webext/icons/icon.svg -y -vf scale=19:19 webext/icons/icon-19.png
	cd webext; pkgx npx -y web-ext run
