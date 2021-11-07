use crate::util::Sender;

use abi_stable::StableAbi;

/// Simplified in this example for simplicity, but the functionality is the
/// same. This struct is used to asynchronously communicate the runtime that the
/// connection has been lost.
#[repr(C)]
#[derive(StableAbi)]
pub struct ConnectionLostNotifier(Sender<()>);
