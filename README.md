# PDK Experiments

This repository contains various experiments written in order to learn more
about how [Tremor's PDK](https://nullderef.com/series/rust-plugins/) could be
implemented.

The structure is usually the following:

* The `src` directory contains the main binary which loads the plugins.
* The interface shared between the main binary and the plugin is defined in
  the `common` crate.
* The plugins live in `plugin-*` directories, which should be compiled as a
  shared library and then loaded from the main binary.

You have to build the main binary and the plugins separately. You can then load
the plugin with the main binary and see what happens. There's usually a
`Makefile` that does everything for you, though.

## Benchmarking

The main crate in `src` includes some benchmarks that might be of interest. It
requires a nightly compiler, and will work as long as the plugins are compiled
in their `target` directories. Remember to use `--release` when building the
plugins.
