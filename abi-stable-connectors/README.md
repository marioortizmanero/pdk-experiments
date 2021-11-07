# abi-stable-connectors

This example represents the Tremor architecture at a small and simplified scale.
It implements the proposed plugins system with a few interesting ideas such as
panic handling and complex types, thanks to the `abi_stable` crate:

- `plugin-metronome`: a source that sends new events every second
- `plugin-reverse`: a sink whose events are lines read from stdin
- `plugin-panic`: an example of safe panicking through the FFI boundary

Based on commit 13526d8 (connectors branch).
