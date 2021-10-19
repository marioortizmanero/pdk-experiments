use abi_stable::{StableAbi, std_types::RString};

#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SinkContext(RString);

#[abi_stable::sabi_trait]
pub trait RawSink {
    // TODO
}
