# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug-metronome:
	cd plugin-metronome && cargo build
	cargo run -- plugin-metronome/target/debug/libmetronome.so
release-metronome:
	cd plugin-metronome && cargo build --release
	cargo run --release -- plugin-metronome/target/release/libmetronome.so

debug: debug-metronome
release: release-metronome

clean:
	rm -rf target plugin-sample/target
