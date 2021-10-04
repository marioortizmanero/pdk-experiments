# Simple Makefile to easily compile both the plugins and the main binary

.PHONY: debug-metronome release-metronome \
	debug-versionmismatch release-versionmismatch \
	debug-wrongtype release-wrongtype \
	debug-missing release-missing \
	debug-wrongaddress release-wrongaddress \
	debug release clean

debug-metronome:
	cd plugin-metronome && cargo build
	cargo run -- plugin-metronome/target/debug/
release-metronome:
	cd plugin-metronome && cargo build --release
	cargo run --release -- plugin-metronome/target/release/

debug-versionmismatch:
	cd plugin-versionmismatch && cargo build
	cargo run -- plugin-versionmismatch/target/debug/
release-versionmismatch:
	cd plugin-versionmismatch && cargo build --release
	cargo run --release -- plugin-versionmismatch/target/release/

debug-wrongtype:
	cd plugin-wrongtype && cargo build
	cargo run -- plugin-wrongtype/target/debug/
release-wrongtype:
	cd plugin-wrongtype && cargo build --release
	cargo run --release -- plugin-wrongtype/target/release/

debug-missing:
	cd plugin-missing && cargo build
	cargo run -- plugin-missing/target/debug/
release-missing:
	cd plugin-missing && cargo build --release
	cargo run --release -- plugin-missing/target/release/

debug-wrongaddress:
	cd plugin-wrongaddress && cargo build
	cargo run -- plugin-wrongaddress/target/debug/
release-wrongaddress:
	cd plugin-wrongaddress && cargo build --release
	cargo run --release -- plugin-wrongaddress/target/release/

debug: debug-metronome
release: release-metronome

clean:
	rm -rf target common/target plugin-*/target
