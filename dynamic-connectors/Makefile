# Simple Makefile to easily compile both the plugins and the main binary

.PHONY: debug-metronome release-metronome \
	debug-versionmismatch release-versionmismatch \
	debug-wrongtype release-wrongtype \
	debug-missing release-missing \
	debug release clean

debug-metronome:
	cd plugin-metronome && cargo build
	cargo run -- plugin-metronome/target/debug/libplugin_metronome.so
release-metronome:
	cd plugin-metronome && cargo build --release
	cargo run --release -- plugin-metronome/target/release/libplugin_metronome.so

debug-versionmismatch:
	cd plugin-versionmismatch && cargo build
	cargo run -- plugin-versionmismatch/target/debug/libplugin_versionmismatch.so
release-versionmismatch:
	cd plugin-versionmismatch && cargo build --release
	cargo run --release -- plugin-versionmismatch/target/release/libplugin_versionmismatch.so

debug-wrongtype:
	cd plugin-wrongtype && cargo build
	cargo run -- plugin-wrongtype/target/debug/libplugin_wrongtype.so
release-wrongtype:
	cd plugin-wrongtype && cargo build --release
	cargo run --release -- plugin-wrongtype/target/release/libplugin_wrongtype.so

debug-missing:
	cd plugin-missing && cargo build
	cargo run -- plugin-missing/target/debug/libplugin_missing.so
release-missing:
	cd plugin-missing && cargo build --release
	cargo run --release -- plugin-missing/target/release/libplugin_missing.so

debug: debug-metronome
release: release-metronome

clean:
	rm -rf target common/target plugin-*/target
