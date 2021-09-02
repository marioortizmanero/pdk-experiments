# Simple Makefile to easily compile both the plugin and the main binary
TARGET=wasm32-wasi
# TARGET=wasm32-unknown-unknown

debug:
	cd plugin-sample && cargo build --target $(TARGET)
	cargo run -- plugin-sample/target/$(TARGET)/debug/plugin_sample.wasm
release:
	cd plugin-sample && cargo build --target $(TARGET) --release
	cargo run -- plugin-sample/target/$(TARGET)/release/plugin_sample.wasm
