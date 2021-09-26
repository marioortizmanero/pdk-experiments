# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug:
	cd plugin-sample && cargo build
	cargo run -- plugin-sample/target/debug/libplugin_sample.so

release:
	cd plugin-sample && cargo build --release
	cargo run --release -- plugin-sample/target/release/libplugin_sample.so

clean:
	rm -rf target plugin-sample/target
