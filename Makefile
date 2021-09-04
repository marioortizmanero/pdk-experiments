# Makefile to run commands for all the examples

CRATES = dynamic-simple wasmer-simple wasmtime-simple

.PHONY: debug release clean

debug: $(CRATES)
	for crate in $^; do cd $$crate && make debug && cd ..; done

release: $(CRATES)
	for crate in $^; do cd $$crate && make release && cd ..; done

clean: $(CRATES)
	for crate in $^; do cd $$crate && make clean && cd ..; done
