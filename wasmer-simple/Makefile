# Simple Makefile to easily compile both the plugin and the main binary
TARGET=wasm32-wasi

.PHONY: debug release clean

debug:
	cd plugin-sample && cargo build --target $(TARGET)
	cargo run -- plugin-sample/target/$(TARGET)/debug/plugin_sample.wasm

release:
	cd plugin-sample && cargo build --target $(TARGET) --release
	cargo run --release -- plugin-sample/target/$(TARGET)/release/plugin_sample.wasm

clean:
	rm -rf target common/target plugin-sample/target
