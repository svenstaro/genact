.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release
	strip target/x86_64-unknown-linux-musl/release/genact
	upx --lzma target/x86_64-unknown-linux-musl/release/genact

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release
	strip target/x86_64-pc-windows-gnu/release/genact.exe
	upx --lzma target/x86_64-pc-windows-gnu/release/genact.exe

.PHONY: build-apple
build-apple:
	cargo build --target x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/genact
	upx --lzma target/x86_64-apple-darwin/release/genact

.PHONY: build-web
build-web:
	trunk build --release

.PHONY: clean
clean:
	cargo clean
	trunk clean
