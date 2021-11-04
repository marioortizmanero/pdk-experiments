mod wrapper {
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
    // This is common
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

    // This is common
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

// TODO: will they work OK in a multi-threaded environment? Perhaps it needs a
// mutex? Something?
fn main() {
    raw::run();
    nicer::run();
}
