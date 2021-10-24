use abi_stable::{StableAbi, std_types::RString};

/// Stub in this example for simplicity.
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectionLostNotifier(RString);
