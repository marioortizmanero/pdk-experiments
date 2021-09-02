use common_ws::MinBuilder;

use std::{slice, mem};

/// Using `pub` is enough to export the function.
///
/// In the case of Wasm, `static` isn't worth using to export the function,
/// since globals can only be integers and it would require some more
/// complicated conversion.
#[no_mangle]
pub fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// This is what the implementation of `with_extern_dyn` would look like if it
/// was possible to import/export complex types.
fn internal_with_extern_dyn(builder: &Box<dyn MinBuilder>, a: i32, b: i32) -> i32 {
    1234
    // builder.min(a, b)
}

/// Wasm can only take integers or floating point values as parameters. This
/// discards both dynamic dispatching and generics.
///
/// Instead, this will use a pointer in Wasm's shared memory, with which the
/// data can be accessed.
#[no_mangle]
pub fn with_extern_dyn(builder_ptr: i32, a: i32, b: i32) -> i32 {
    println!("uwu");
    dbg!();
    // Extract the data from memory
    let builder: Box<dyn MinBuilder> =  unsafe {
        // We can know the size of the data because the trait is object-safe and
        // always the same.
        let size = mem::size_of::<Box<dyn MinBuilder>>();

        // We extract from memory the data
        let data = slice::from_raw_parts(builder_ptr as _, size);

        // let options = bincode::DefaultOptions::new();
        // let mut deserializer = bincode::de::Deserializer::from_slice(data, options);
        // let mut objsafe_deserializer = Box::new(<dyn erased_serde::Deserializer>::erase(&mut deserializer));
        // let mut objsafe_deserializer = erased_serde::Deserializer::erase(&mut deserializer);

        // And we deserialize it into the object we need. Note that we use
        // `deserialize_from` and not `deserialize` because `MinBuilder`
        // requires `serde::de::DeserializeOwned` instead of
        // `serde::de::Deserialize<'a>` in order to be object-safe.
        // erased_serde::deserialize(&mut objsafe_deserializer).unwrap()

        match bincode::deserialize(data) {
            Ok(d) => d,
            Err(e) => {
                println!("ERROR {}\n", e);
                return 666
            },
        }
    };

    internal_with_extern_dyn(&builder, a, b)
}
