use super::{StableAbi, RString};

#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectionLostNotifier(RString);
