use abi_stable::{sabi_trait, std_types::RBox, type_level::downcasting::TD_Opaque};

#[sabi_trait]
pub trait State: Debug {
    fn counter(&self) -> i32;
}

// A trait object for `State`
pub type StateBox = State_TO<'static, RBox<()>>;

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

    impl State for MyState {
        fn counter(&self) -> i32 {
            self.counter
        }
    }
}

fn main() {
    use sample_impl::MyState;

    let state = MyState { counter: 0 };
    let mut state = State_TO::from_value(state, TD_Opaque);
    usage(&mut state);
}
