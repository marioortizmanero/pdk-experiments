# abi-stable-connectors

This example represents the Tremor architecture at a small and simplified scale.
It implements the proposed plugins system with a few interesting ideas such as
panic handling and complex types, thanks to the `abi_stable` crate:

- `plugin-metronome`: a source that sends new events every second
- `plugin-reverse`: a sink whose events are lines read from stdin
- `plugin-panic`: an example of safe panicking through the FFI boundary

Based on commit 13526d8 (connectors branch).

Note that this project is merely experimental; it's not really exactly how your
plugin system should look in the end:

- It uses `MayPanic` only sometimes to showcase how it works, but it's not
  really necessary here anyway.
- It uses `FfiFuture` only sometimes as well to learn its usage.
