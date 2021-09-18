use common_dconnectors::{ConnectorPlugin, define_connector_plugin};

use abi_stable::{rslice, rstr, std_types::RSliceMut};

// TODO

define_connector_plugin! {
    name: "metronome",
    data: ConnectorPlugin {
        something: metronome,
        mime_types: rslice![rstr!("json")]
    }
}

unsafe extern fn metronome<'input>(_data: RSliceMut<'input, u8>) -> i32 {
    1234
}
