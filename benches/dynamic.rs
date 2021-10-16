//! These benchmarks are more reliable than the simple ones, and can be used to
//! more or less find out the performance difference between using raw dynamic
//! loading and abi_stable.

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

const DYNAMIC_PATH: &str = "dynamic-bench/plugin-sample/target/release/";
const ABI_STABLE_PATH: &str = "abi-stable-bench/plugin-sample/target/release/";

fn dynamic_setup(bench: &mut Criterion) {
    check_exists(DYNAMIC_PATH);
    bench.bench_function("dynamic setup", move |b| {
        b.iter(|| {
            let _ = dynamic_bench::setup_plugin(DYNAMIC_PATH).unwrap();
        })
    });
}

fn abi_stable_setup(bench: &mut Criterion) {
    check_exists(ABI_STABLE_PATH);
    bench.bench_function("abi_stable setup", move |b| {
        b.iter(|| {
            let _ = abi_stable_bench::setup_plugin(ABI_STABLE_PATH).unwrap();
        })
    });
}

fn dynamic_runtime(bench: &mut Criterion) {
    check_exists(DYNAMIC_PATH);
    let mut run_fn = dynamic_bench::setup_plugin(DYNAMIC_PATH).unwrap();
    bench.bench_function("dynamic runtime", move |b| {
        let mut n1 = 100;
        let mut n2 = -100;
        b.iter(|| {
            let ret = run_fn(n1, n2);
            n1 -= ret;
            n2 += ret;
        })
    });
}

fn abi_stable_runtime(bench: &mut Criterion) {
    check_exists(ABI_STABLE_PATH);
    let mut run_fn = abi_stable_bench::setup_plugin(ABI_STABLE_PATH).unwrap();
    bench.bench_function("abi_stable runtime", move |b| {
        let mut n1 = 100;
        let mut n2 = -100;
        b.iter(|| {
            let ret = run_fn(n1, n2);
            n1 -= ret;
            n2 += ret;
        })
    });
}

fn native_runtime(bench: &mut Criterion) {
    bench.bench_function("native runtime", move |b| {
        let mut n1 = 100;
        let mut n2 = -100;
        b.iter(|| {
            let ret = n1.min(n2);
            n1 -= ret;
            n2 += ret;
        })
    });
}

criterion_group!(setup, dynamic_setup, abi_stable_setup);
criterion_group!(runtime, dynamic_runtime, abi_stable_runtime, native_runtime);
criterion_main!(setup, runtime);
