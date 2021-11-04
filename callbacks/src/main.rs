mod wrapper {
    #[derive(Clone)]
    pub struct Sender<T> {
        callback: fn(T),
    }

    impl<T> Sender<T> {
        pub fn new(callback: fn(T)) -> Self {
            Self {
                callback
            }
        }

        pub fn send(&self, t: T) {
            (self.callback)(t);
        }
    }
}

mod raw {
    // This is in common
    pub struct ConnectorContext {
        callback: fn(i32),
    }

    // This is the plugin
    pub fn plugin_fn(ctx: &ConnectorContext) {
        (ctx.callback)(1234);
    }

    // This is the runtime
    pub fn run() {
        let ctx = ConnectorContext {
            callback: |x| println!("callback invoked v1! {}", x)
        };
        plugin_fn(&ctx);
    }
}

mod nicer {
    use super::wrapper::Sender;

    // This is in common
    pub struct ConnectorContext {
        sender: Sender<i32>
    }

    // This is the plugin
    pub fn plugin_fn(ctx: &ConnectorContext) {
        ctx.sender.send(1234);
    }

    // This is the runtime
    pub fn run() {
        let ctx = ConnectorContext {
            sender: Sender::new(|x| println!("callback invoked v2! {}", x))
        };
        plugin_fn(&ctx);
    }
}

mod nicer_concurrent {
    use super::wrapper::Sender;
    use std::{thread, time::Duration};

    // This is in common
    #[derive(Clone)]
    pub struct ConnectorContext {
        sender: Sender<i32>
    }

    // This is the plugin
    pub fn plugin_fn(ctx: &ConnectorContext) {
        // The callback will be invoked a few times at about the same time
        for i in 0..5 {
            let ctx = ctx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                ctx.sender.send(i);
            });
        }
    }

    pub fn run() {
        // NOTE: This will work as long as the function in the `Sender` can be
        // coerced into a regular function (i.e. it doesn't capture anything).
        let ctx = ConnectorContext {
            sender: Sender::new(|x| {
                println!("callback invoked v3! {}", x)
            })
        };
        // Waiting for the callbacks to occur
        plugin_fn(&ctx);
        thread::sleep(Duration::from_millis(500));
    }
}

// TODO: will they work OK in a multi-threaded environment? Perhaps it needs a
// mutex? Something?
fn main() {
    raw::run();
    nicer::run();
    nicer_concurrent::run();
}
