use abi_stable::{rstr, std_types::RStr, StableAbi};

/// This is the struct that's passed to the functions in the module.
#[repr(C)]
#[derive(StableAbi)]
pub struct State {
}

// Using the stable C ABI
#[repr(C)]
// Deriving the `StableAbi` trait, which defines the layout of the struct at
// compile-time.
#[derive(StableAbi)]
// Marking the struct as a prefix-type.
#[sabi(kind(prefix_ref))]
pub struct MinMod {
    /// Initializes the state, which will be passed to the functions in this
    /// module.
    pub new: extern "C" fn() -> State,

    /// Reverses the order of the lines.
    pub min: extern "C" fn(&mut TOStateBox,RStr<'_>) -> RString,
 
    pub run_command: 
        extern "C" fn(&mut TOStateBox,command:TOCommandBox<'static>)->TOReturnValueArc,
}

/// This symbol is exported using `abi_stable` v0.9
#[no_mangle]
pub static SHARED: RStr = rstr!("hello");
