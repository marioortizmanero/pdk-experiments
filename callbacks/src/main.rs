const NUM_THREADS: i32 = 10;

mod raw {
    use super::NUM_THREADS;
    use std::sync::atomic::{AtomicI32, Ordering};

    // This type is shared by the plugin and the runtime
    #[repr(C)]
    struct ConnectorContext {
        id: i32,
        callback: extern "C" fn(i32),
    }

    // This is the plugin. It only invokes the callback with its own ID.
    extern "C" fn plugin_fn(ctx: &ConnectorContext) {
        (ctx.callback)(ctx.id);
    }

    // This is the runtime. It will run a few plugins concurrently.
    pub fn run() {
        // You still have access to some resources in the runtime, but don't
        // abuse it.
        static COUNT: AtomicI32 = AtomicI32::new(0);
        extern "C" fn callback(x: i32) {
            println!("raw callback invoked! {x:?}");
            COUNT.fetch_add(1, Ordering::Relaxed);
        }

        let mut handles = Vec::new();
        for id in 0..NUM_THREADS {
            handles.push(std::thread::spawn(move || {
                let ctx = ConnectorContext { id, callback };
                plugin_fn(&ctx);
            }))
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Final count: {}", COUNT.load(Ordering::Relaxed));
    }
}

mod nicer {
    mod wrapper {
        pub struct Sender<T> {
            callback: extern "C" fn(T),
        }

        impl<T> Sender<T> {
            pub fn new(callback: extern "C" fn(T)) -> Self {
                Self { callback }
            }

            pub fn send(&self, t: T) {
                (self.callback)(t);
            }
        }
    }

    use super::NUM_THREADS;
    use wrapper::Sender;

    // This type is shared by the plugin and the runtime
    #[repr(C)]
    struct ConnectorContext {
        id: i32,
        sender: Sender<i32>,
    }

    // This is the plugin. It only invokes the callback with its own ID.
    extern "C" fn plugin_fn(ctx: &ConnectorContext) {
        ctx.sender.send(ctx.id);
    }

    // This is the runtime. It will run a few plugins concurrently.
    pub fn run() {
        extern "C" fn callback(x: i32) {
            println!("nicer callback invoked! {x}")
        }

        let mut handles = Vec::new();
        for id in 0..NUM_THREADS {
            handles.push(std::thread::spawn(move || {
                let ctx = ConnectorContext {
                    id,
                    sender: Sender::new(callback),
                };
                plugin_fn(&ctx);
            }))
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    raw::run();
    nicer::run();
}
