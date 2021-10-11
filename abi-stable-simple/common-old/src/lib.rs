//! This crate is the same as `common`, but pretending to be a previous version
//! in order to load it with `plugin-versionmismatch. The only difference is
//! that `State` has a field with `bool` instead of `i32`.

use abi_stable::{StableAbi, declare_root_module_statics, library::RootModule, package_version_strings, sabi_types::VersionStrings};

/// This is the struct that's passed to the functions in the module, which
/// serves as a persistent state in a safer way than with a global.
#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct State {
    pub has_ran: bool
}

// Using the stable C ABI
#[repr(C)]
// Deriving the `StableAbi` trait, which defines the layout of the struct at
// compile-time:
// https://docs.rs/abi_stable/0.10.2/abi_stable/derive.StableAbi.html
#[derive(StableAbi)]
// Marking the struct as a prefix-type:
// https://docs.rs/abi_stable/0.10.2/abi_stable/docs/prefix_types/index.html
#[sabi(kind(Prefix))]
pub struct MinMod {
    /// Initializes the state, which will be passed to the functions in this
    /// module.
    pub new: extern "C" fn() -> State,

    /// Calculates the minimum between two integers
    pub min: extern "C" fn(&mut State, i32, i32) -> i32,
}

// Marking `MinMod` as the main module in this plugin. Note that `MinMod_Ref` is
// a pointer to the prefix of `MinMod`.
impl RootModule for MinMod_Ref {
    // The name of the dynamic library
    const BASE_NAME: &'static str = "min";
    // The name of the library for logging and similars
    const NAME: &'static str = "min";
    // The version of this plugin's crate
    const VERSION_STRINGS: VersionStrings = package_version_strings!();

    // Implements the `RootModule::root_module_statics` function, which is the
    // only required implementation for the `RootModule` trait.
    declare_root_module_statics!{MinMod_Ref}
}
