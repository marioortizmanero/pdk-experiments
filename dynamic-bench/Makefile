# Simple Makefile to easily compile both the plugins and the main binary

.PHONY: debug-sample release-sample

debug-sample:
	cd plugin-sample && cargo build
	cargo build
release-sample:
	cd plugin-sample && cargo build --release
	cargo build --release

debug: debug-sample
release: release-sample

clean:
	rm -rf target common/target plugin-*/target
