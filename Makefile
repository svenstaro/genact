.PHONY: build
build-linux:
	cargo build --target x86_64-unknown-linux-musl

.PHONY: build-win
build-win:
	cargo build --target x86_64-pc-windows-gnu

.PHONY: build-apple
build-apple:
	cargo build --target x86_64-apple-darwin
