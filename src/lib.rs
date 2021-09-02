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
                plugin before running the benchmarks.",
                path
            )
        }
    }

    const DYNAMIC_PATH: &str = "dynamic-simple/plugin-sample/target/release/libplugin_sample.so";
    const WASM_PATH: &str =
        "wasm-simple/plugin-sample/target/wasm32-unknown-unknown/release/plugin_sample.wasm";

    #[bench]
    fn dynamic_setup(bench: &mut Bencher) {
        check_exists(DYNAMIC_PATH);
        bench.iter(|| dynamic_bench::setup(WASM_PATH).unwrap())
    }

    #[bench]
    fn dynamic_runtime(bench: &mut Bencher) {
        check_exists(DYNAMIC_PATH);
        let library = dynamic_bench::setup(WASM_PATH).unwrap();
        bench.iter(|| dynamic_bench::run_plugin(library).unwrap())
    }

    #[bench]
    fn wasm_setup(bench: &mut Bencher) {
        check_exists(WASM_PATH);
        bench.iter(|| wasm_bench::setup(WASM_PATH).unwrap())
    }

    #[bench]
    fn wasm_runtime(bench: &mut Bencher) {
        check_exists(WASM_PATH);
        let library = wasm_bench::setup(WASM_PATH).unwrap();
        bench.iter(|| wasm_bench::run_plugin(library).unwrap())
    }
}
