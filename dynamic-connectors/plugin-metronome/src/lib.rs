use common_dconnectors::{define_connector_plugin, ConnectorPlugin};

use abi_stable::{rslice, rstr};

use std::{mem::ManuallyDrop, ffi::c_void};

define_connector_plugin! {
    name: "metronome",
    data: ConnectorPlugin {
        mime_types: rslice![rstr!("json")],
        new,
        something,
    }
}

pub struct MetronomeState {
    counter: i32
}

unsafe extern "C" fn new() -> *mut c_void {
    // We need the pointer to be alive after this function
    let mut state = ManuallyDrop::new(MetronomeState {
        counter: 0
    });

    &mut (*state) as *mut MetronomeState as _
}

unsafe extern "C" fn something(state: *mut c_void) -> i32 {
    // Casting the void pointer to the original type
    let state: &mut MetronomeState = &mut *(state as *mut MetronomeState);

    state.counter += 1;

    state.counter
}
