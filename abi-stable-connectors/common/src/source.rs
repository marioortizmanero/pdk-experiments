use abi_stable::{std_types::RString, StableAbi};

use crate::{RResult, util::MayPanic};

/// Stub for now
#[repr(C)]
#[derive(StableAbi, Default, Clone)]
pub struct SourceContext(RString);

/// Simplified for now
#[repr(C)]
#[derive(StableAbi)]
pub enum SourceReply {
    Empty(u64), // contains the milliseconds to sleep for the next iteration
    Data(RString), // should be a vector of u8 or similars
}

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
