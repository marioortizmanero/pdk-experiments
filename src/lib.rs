#![feature(test)]

#[cfg(test)]
mod test {
    extern crate test;

    use std::path::Path;

    use test::Bencher;

    fn check_exists(path: &str) {
        if !Path::new(path).exists() {
            panic!(
                "The path '{}' doesn't exist. Make sure you've compiled the plugin before running the benchmarks.",
                path
            )
        }
    }

    const DYNAMIC_PATH: &str = "dynamic-simple/plugin-sample/target/release/libplugin_sample.so";
    const WASM_PATH: &str = "wasm-simple/plugin-sample/target/wasm32-unknown-unknown/release/plugin_sample.wasm";

    #[bench]
    fn dynamic_simple(bench: &mut Bencher) {
        check_exists(DYNAMIC_PATH);
        bench.iter(|| dynamic_simple::run_plugin(DYNAMIC_PATH, true).unwrap())
    }

    #[bench]
    fn wasm_simple(bench: &mut Bencher) {
        check_exists(WASM_PATH);
        bench.iter(|| wasm_simple::run_plugin(WASM_PATH).unwrap())
    }
}
