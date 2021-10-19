use super::{RString, StableAbi};

#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectionLostNotifier(RString);
