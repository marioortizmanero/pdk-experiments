use abi_stable::{export_root_module, sabi_extern_fn, prefix_type::PrefixTypeTrait};

use common_sabi_simple::{MinMod, MinMod_Ref, State};

/// Exports the root module of this library.
///
/// This code isn't run until the layout of the type it returns is checked.
#[export_root_module]
fn instantiate_root_module() -> MinMod_Ref {
    MinMod {
        new,
        min
    }
    .leak_into_prefix() // Converts the `MinMod` into `MinMod_Ref` and leaks it
}

// TODO: research "erasing"
#[sabi_extern_fn]
pub fn new() -> State {
    State {
        counter: 0
    }
}

#[sabi_extern_fn]
pub fn min(state: &mut State, a: i32, b: i32) -> i32 {
    state.counter += 1;
    a.min(b)
}
