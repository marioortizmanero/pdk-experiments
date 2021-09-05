#![feature(test)]

/// TODO: docs
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

    // These are not really fair, but it's interesting to see the difference.
    // Note that it benchmarks both the initialization of the plugin and the
    // runtime.

    mod paths {
        pub const DYNAMIC_SIMPLE: &str =
            "dynamic-simple/plugin-sample/target/release/libplugin_sample.so";
        pub const WASMER_SIMPLE: &str =
            "wasmer-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";
        pub const WASMTIME_SIMPLE: &str =
            "wasmtime-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";
        pub const DYNAMIC_CODEC: &str =
            "dynamic-codec/plugin-sample/target/release/libplugin_sample.so";
    }

    #[bench]
    fn dynamic_simple(bench: &mut Bencher) {
        check_exists(paths::DYNAMIC_SIMPLE);
        bench.iter(|| dynamic_simple::run_plugin(paths::DYNAMIC_SIMPLE).unwrap())
    }

    #[bench]
    fn wasmer_setup(bench: &mut Bencher) {
        check_exists(paths::WASMER_SIMPLE);
        bench.iter(|| wasmer_simple::run_plugin(paths::WASMER_SIMPLE).unwrap())
    }

    #[bench]
    fn wasmtime_setup(bench: &mut Bencher) {
        check_exists(paths::WASMTIME_SIMPLE);
        bench.iter(|| wasmer_simple::run_plugin(paths::WASMTIME_SIMPLE).unwrap())
    }

    #[bench]
    fn dynamic_codec_setup(bench: &mut Bencher) {
        check_exists(paths::DYNAMIC_CODEC);
        bench.iter(|| dynamic_codec::setup_plugin(paths::DYNAMIC_CODEC).unwrap())
    }

    #[bench]
    fn dynamic_codec_runtime(bench: &mut Bencher) {
        check_exists(paths::DYNAMIC_CODEC);
        let run_plugin = dynamic_codec::setup_plugin(paths::DYNAMIC_CODEC).unwrap();
        bench.iter(move || run_plugin().unwrap())
    }

    #[bench]
    fn native_codec_setup(bench: &mut Bencher) {
        bench.iter(|| native_codec::setup_plugin())
    }

    #[bench]
    fn native_codec_runtime(bench: &mut Bencher) {
        let run_plugin = native_codec::setup_plugin();
        bench.iter(run_plugin)
    }
}
