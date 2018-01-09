.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release

.PHONY: build-apple
build-apple:
	cargo build --target x86_64-apple-darwin --release

.PHONY: build-web
build-web:
	EMMAKEN_CFLAGS="-s ASYNCIFY=1" cargo build --target wasm32-unknown-emscripten --release

.PHONY: run-web
run-web:
	EMMAKEN_CFLAGS="-s ASYNCIFY=1" cargo web start --target-wasm-emscripten --use-system-emscripten --release
