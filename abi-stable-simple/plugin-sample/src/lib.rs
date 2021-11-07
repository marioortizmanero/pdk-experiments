use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn,
    type_level::downcasting::TD_Opaque,
};

use common_sabi_simple::{MinMod, MinMod_Ref, State, StateBox, State_TO};

#[derive(Clone, Debug)]
struct SampleState {
    counter: i32,
}
impl State for SampleState {
    fn incr_counter(&mut self) {
        self.counter += 1
    }
    fn counter(&self) -> i32 {
        self.counter
    }
}

/// Exports the root module of this library.
///
/// This code isn't run until the layout of the type it returns is checked.
#[export_root_module]
fn instantiate_root_module() -> MinMod_Ref {
    MinMod { new, min }.leak_into_prefix() // Converts the `MinMod` into `MinMod_Ref` and leaks it
}

#[sabi_extern_fn]
pub fn new() -> StateBox {
    let state = SampleState { counter: 0 };
    State_TO::from_value(state, TD_Opaque)
}

#[sabi_extern_fn]
pub fn min(state: &mut StateBox, a: i32, b: i32) -> i32 {
    state.incr_counter();
    a.min(b)
}
