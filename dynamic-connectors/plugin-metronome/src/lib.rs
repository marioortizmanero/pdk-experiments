use common_dconnectors::{define_connector_plugin, ConnectorPlugin};

use abi_stable::{rslice, rstr, std_types::RSliceMut};

define_connector_plugin! {
    name: "metronome",
    data: ConnectorPlugin {
        something: metronome,
        mime_types: rslice![rstr!("json")]
    }
}

unsafe extern "C" fn metronome<'input>(_data: RSliceMut<'input, u8>) -> i32 {
    1234
}
