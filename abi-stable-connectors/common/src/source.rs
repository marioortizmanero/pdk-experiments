use abi_stable::{std_types::RString, StableAbi};

use crate::{RResult, util::MayPanic};

// Stubs for the original trait. We can't use `()` because it's not FFI-safe.
#[repr(C)]
#[derive(StableAbi, Default, Clone)]
pub struct SourceContext(RString);

#[repr(C)]
#[derive(StableAbi)]
pub enum SourceReply {
    Empty(u64), // contains the milliseconds to sleep for the next iteration
    Data(RString), // should be a vector of u8 or similars
}

// The source needs a trait object because it's now returned in the FFI
// interface.
#[abi_stable::sabi_trait]
pub trait RawSource: Send {
    /// Pulls an event from the source if one exists
    /// determine the codec to be used
    fn pull_data(&mut self, pull_id: u64, ctx: &SourceContext) -> MayPanic<RResult<SourceReply>>;

    /// Is this source transactional or can acks/fails be ignored
    fn is_transactional(&self) -> bool {
        false
    }
}
