#!/usr/bin/env python3

import os
import subprocess
from timeit import timeit

cmdkwargs = {
    "stdout": subprocess.DEVNULL,
    "stderr": subprocess.DEVNULL,
    "check": True
}

print("Compiling dynamic-simple...")
os.chdir("dynamic-simple")
subprocess.run(["cargo", "build", "--release"], **cmdkwargs)
os.chdir("plugin-sample")
subprocess.run(["cargo", "build", "--release"], **cmdkwargs)
os.chdir("../..")

print("Compiling wasm-simple...")
os.chdir("wasm-simple")
subprocess.run(["cargo", "build", "--release"], **cmdkwargs)
os.chdir("plugin-sample")
subprocess.run(["cargo", "build", "--target", "wasm32-unknown-unknown", "--release"], **cmdkwargs)
os.chdir("../..")

dynamic_cmd = [
    "./dynamic-simple/target/release/pdk-experiments",
    "./dynamic-simple/plugin-sample/target/release/libplugin_sample.so",
    "--simple-mode"
]
wasm_cmd = [
    "./dynamic-simple/target/release/pdk-experiments",
    "./dynamic-simple/plugin-sample/target/release/libplugin_sample.so",
    "--simple-mode"
]
print("Benchmarking...")
for cmd in (dynamic_cmd, wasm_cmd):
    t = timeit(f"subprocess.run(cmd, **cmdkwargs)", globals=globals(), number=100)
    print(t)
