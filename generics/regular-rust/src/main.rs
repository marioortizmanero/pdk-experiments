use std::fmt::Debug;

pub trait State: Debug {
    fn counter(&self) -> i32;
}

// It can then be used easily like this
pub fn usage(state: &mut Box<dyn State>) {
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
    let mut state = Box::new(state) as Box<dyn State>;
    usage(&mut state);
}
