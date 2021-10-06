# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug-sample:
	cd plugin-sample && cargo build
	cargo run -- plugin-sample/target/debug/libplugin_sample.so
release-sample:
	cd plugin-sample && cargo build --release
	cargo run --release -- plugin-sample/target/release/libplugin_sample.so

debug-panic:
	cd plugin-panic && cargo build
	cargo run -- plugin-panic/target/debug/libplugin_panic.so
release-panic:
	cd plugin-panic && cargo build --release
	cargo run --release -- plugin-panic/target/release/libplugin_panic.so

clean:
	rm -rf target plugin-sample/target
