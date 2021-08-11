# PDK Experiments

This repository contains various experiments written in order to learn more
about how [Tremor's PDK](https://nullderef.com/blog/gsoc-proposal/) could be
implemented.

The structure is usually the following:

* The `src` directory contains the main binary which loads the plugins.
* The interface shared between the main binary and the plugin is defined in
  the `tremor_core` crate.
* The plugins live in `plugin-*` directories, which should be compiled as a
  shared library and then loaded from the main binary.

You have to build the main binary and the plugins separately. You can then load
the plugin with the main binary and see what happens.
