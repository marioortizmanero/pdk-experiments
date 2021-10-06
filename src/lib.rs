#![feature(test)]

#[cfg(test)]
mod test {
    extern crate test;

    use std::path::Path;

    use test::Bencher;

    fn check_exists(path: &str) {
        if !Path::new(path).exists() {
            panic!(
                "The path '{}' doesn't exist. Make sure you've compiled the \
                plugin before running the benchmarks. You can do that with \
                `make release` in the repository directory",
                path
            )
        }
    }

    /*
     * Note that the simple benchmarks are not really fair, but it's interesting
     * to see the difference. They measure both the initialization of the plugin
     * and the runtime.
     */

    const DYNAMIC_PATH: &str = "dynamic-simple/plugin-sample/target/release/libplugin_sample.so";
    const WASMER_PATH: &str =
        "wasmer-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";
    const WASMTIME_PATH: &str =
        "wasmtime-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";

    #[bench]
    fn dynamic_simple(bench: &mut Bencher) {
        check_exists(DYNAMIC_PATH);
        bench.iter(|| dynamic_simple::run_plugin(DYNAMIC_PATH).unwrap())
    }

    #[bench]
    fn wasmer_setup(bench: &mut Bencher) {
        check_exists(WASMER_PATH);
        bench.iter(|| wasmer_simple::run_plugin(WASMER_PATH).unwrap())
    }

    #[bench]
    fn wasmtime_setup(bench: &mut Bencher) {
        check_exists(WASMTIME_PATH);
        bench.iter(|| wasmer_simple::run_plugin(WASMTIME_PATH).unwrap())
    }
}
