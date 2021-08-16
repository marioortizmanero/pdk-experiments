# wasm-simple

This example showcases the simplest possible Wasm plugin architecture. It
requires a slightly more complicated setup because the plugin has to be built
for the Wasm platform with the following command instead:

```
cargo build --target wasm32-unknown-unknown
```

For which you might need first:

```
rustup target add wasm32-unknown-unknown
```

The plugin will be in the directory
`target/wasm32-unknown-unknown/debug/plugin_sample.wasm`, which can be loaded
with the main program.
