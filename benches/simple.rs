//! Note that the simple benchmarks are not really fair, but it's interesting
//! to see the difference. They measure both the initialization of the plugin
//! and the runtime.

use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

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

const DYNAMIC_PATH: &str = "dynamic-simple/plugin-sample/target/release/libplugin_sample.so";
const WASMER_PATH: &str =
    "wasmer-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";
const WASMTIME_PATH: &str =
    "wasmtime-simple/plugin-sample/target/wasm32-wasi/release/plugin_sample.wasm";

fn dynamic_simple(bench: &mut Criterion) {
    check_exists(DYNAMIC_PATH);
    bench.bench_function("dynamic simple", |b| {
        b.iter(|| dynamic_simple::run_plugin(DYNAMIC_PATH).unwrap())
    });
}

fn wasmer_simple(bench: &mut Criterion) {
    check_exists(WASMER_PATH);
    bench.bench_function("wasmer simple", |b| {
        b.iter(|| wasmer_simple::run_plugin(WASMER_PATH).unwrap())
    });
}

fn wasmtime_simple(bench: &mut Criterion) {
    check_exists(WASMTIME_PATH);
    bench.bench_function("wasmtime simple", |b| {
        b.iter(|| wasmer_simple::run_plugin(WASMTIME_PATH).unwrap())
    });
}

criterion_group!(benches, dynamic_simple, wasmer_simple, wasmtime_simple);
criterion_main!(benches);
