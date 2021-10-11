# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug-sample:
	cd plugin-sample && cargo build
	cargo run -- plugin-sample/target/debug/
release-sample:
	cd plugin-sample && cargo build --release
	cargo run --release -- plugin-sample/target/release/

debug-panic:
	cd plugin-panic && cargo build
	cargo run -- plugin-panic/target/debug/
release-panic:
	cd plugin-panic && cargo build --release
	cargo run --release -- plugin-panic/target/release/

debug-versionmismatch:
	cd plugin-versionmismatch && cargo build
	cargo run -- plugin-versionmismatch/target/debug/
release-versionmismatch:
	cd plugin-versionmismatch && cargo build --release
	cargo run --release -- plugin-versionmismatch/target/release/

clean:
	rm -rf target plugin-sample/target
