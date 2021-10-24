//! TODO: not implemented for now

use abi_stable::{std_types::RString, StableAbi};

#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SinkContext(RString);

#[abi_stable::sabi_trait]
pub trait RawSink {}
