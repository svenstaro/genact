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
	cargo web build --target-webasm-emscripten --use-system-emscripten --release

.PHONY: run-web
run-web:
	cargo web start --target-webasm-emscripten --use-system-emscripten --release
