#![allow(non_upper_case_globals, non_camel_case_types)] // For warnings inside `abi_stable` derives

pub mod connectors;
pub mod reconnect;
pub mod sink;
pub mod source;
pub mod util;
pub mod event;
pub mod value;

use crate::{connectors::{RawConnector_TO, TremorUrl}, value::Value};

use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RBox, RBoxError, ROption},
    StableAbi,
};

// For ease of use. Both are equivalent, but `RResult` is used in the
// `abi_stable` context.
pub type RResult<T> = abi_stable::std_types::RResult<T, RBoxError>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Constants
pub const DEFAULT_STREAM_ID: u64 = 0;

/// The `new` function is basically the `ConnectorBuilder::from_config` method.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
pub struct ConnectorMod {
    pub new: extern "C" fn(id: &TremorUrl, config: ROption<Value>) -> RawConnector_TO<'static, RBox<()>>,
}

// Marking `MinMod` as the main module in this plugin. Note that `MinMod_Ref` is
// a pointer to the prefix of `MinMod`.
impl RootModule for ConnectorMod_Ref {
    // The name of the dynamic library
    const BASE_NAME: &'static str = "connector";
    // The name of the library for logging and similars
    const NAME: &'static str = "connector";
    // The version of this plugin's crate
    const VERSION_STRINGS: VersionStrings = package_version_strings!();

    // Implements the `RootModule::root_module_statics` function, which is the
    // only required implementation for the `RootModule` trait.
    declare_root_module_statics! {ConnectorMod_Ref}
}
