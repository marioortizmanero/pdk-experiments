use common_dconnectors::{PluginData, define_plugin};

use abi_stable::{rslice, rstr, std_types::RSliceMut};

// TODO

define_plugin! {
    name: "metronome",
    data: PluginData::Connector {
        something: metronome,
        mime_types: rslice![rstr!("json")]
    }
}

unsafe extern fn metronome<'input>(_data: RSliceMut<'input, u8>) -> i32 {
    1234
}
