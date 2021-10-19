#![allow(
    non_upper_case_globals,
    non_camel_case_types,
)] // For warnings inside `abi_stable` derives

pub mod connectors;
pub mod source;
pub mod sink;
mod wrappers;

use abi_stable::{
    declare_root_module_statics, library::RootModule, package_version_strings,
    sabi_types::VersionStrings, std_types::{RBox, RString}, StableAbi,
};

use crate::connectors::RawConnector_TO;

// For ease of use
pub type RResult<T> = abi_stable::std_types::RResult<T, RString>;
pub type Result<T> = std::result::Result<T, RString>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
pub struct ConnectorMod {
    pub new: extern "C" fn() -> RawConnector_TO<'static, RBox<()>>,
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
