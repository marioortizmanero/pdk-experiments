use abi_stable::{
    erased_types::{ImplType, TypeInfo},
    impl_get_type_info,
    std_types::RBox,
    DynTrait, StableAbi,
};

#[repr(C)]
#[derive(StableAbi)]
// An `InterfaceType` describes which traits are required when constructing
// `StateBox` and are then usable afterwards.
#[sabi(impl_InterfaceType(Debug, PartialEq))]
pub struct State;

// A trait object for `State`
pub type StateBox = DynTrait<'static, RBox<()>, State>;

// It can then be used easily like this
pub fn usage(state: &mut StateBox) {
    println!("state debug: {:?}", state);
}

mod sample_impl {
    use super::*;

    // We derive the traits required by the interface type
    #[derive(Debug, PartialEq)]
    pub struct MyState {
        pub counter: i32,
    }

    /// Declares TOState as the `ìnterface type` of `TextOperationState`.
    ///
    /// Also declares the INFO constant,with information about the type,
    /// used when erasing/unerasing the type with `DynTrait<_>`.
    ///
    /// TOState defines which traits are required when constructing DynTrait<_>,
    /// and which ones it provides after constructing it.
    impl ImplType for MyState {
        type Interface = State;

        const INFO: &'static TypeInfo = impl_get_type_info! { MyState };
    }
}

fn main() {
    use sample_impl::MyState;

    let state = MyState {
        counter: 0
    };
    let mut state = DynTrait::from_value(state);
    usage(&mut state);
}
