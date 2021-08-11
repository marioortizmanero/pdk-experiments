# dynamic-simple

This example is the simplest plugin I could think of that is both written and
loaded with Rust. Do note that this doesn't enforce any constraints on what
plugins are supported. You'll have to make sure both parts of the program are
compiled with the same version of rustc and the `core` library.

There are of course ways to make the plugin architecture more "rusty" (using
structs and traits, for instance), which is covered
[here](https://adventures.michaelfbryan.com/posts/plugins-in-rust/#determining-the-plugin-interface)
and in other examples of this repository, but this attempts to keep it simple in
order to understand the basics.
