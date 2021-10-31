//! This should actually be the tremor-value crate. Simplified for this example.

use abi_stable::{StableAbi, std_types::RString};

#[repr(C)]
#[derive(StableAbi)]
pub enum Value {
    String(RString),
    Integer(i32),
}
