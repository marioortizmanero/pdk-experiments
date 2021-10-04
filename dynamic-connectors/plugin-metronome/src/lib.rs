use common_dconnectors::{define_connector_plugin, ConnectorPlugin};

use std::{mem::ManuallyDrop, ffi::c_void};

define_connector_plugin! {
    name: "metronome",
    data: ConnectorPlugin {
        new,
        something,
        is_sink: false,
        is_source: true
    }
}

pub struct MetronomeState {
    counter: i32
}

unsafe extern "C" fn new() -> *mut c_void {
    // We need the pointer to be alive after this function
    let mut state = ManuallyDrop::new(Box::new(MetronomeState {
        counter: 0
    }));

    &mut (**state) as *mut MetronomeState as _
}

unsafe extern "C" fn something(state: *mut c_void) -> i32 {
    // Casting the void pointer to the original type
    let state = &mut *(state as *mut MetronomeState);

    state.counter += 1;

    state.counter
}
