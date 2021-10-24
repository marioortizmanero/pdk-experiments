# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug: debug-metronome
release: release-metronome

debug-metronome:
	cd plugin-metronome && cargo build
	cargo run -- plugin-metronome/target/debug/libmetronome.so
release-metronome:
	cd plugin-metronome && cargo build --release
	cargo run --release -- plugin-metronome/target/release/libmetronome.so

debug-panic:
	cd plugin-panic && cargo build
	cargo run -- plugin-panic/target/debug/libpanic.so
release-panic:
	cd plugin-panic && cargo build --release
	cargo run --release -- plugin-panic/target/release/libpanic.so

clean:
	rm -rf target plugin-sample/target
