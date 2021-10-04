use std::ffi::c_void;

/// These are the identifiers and types that should be used between the plugin
/// and runtime to import and export functionality.
pub mod interface {
    pub const NAME_IDENT: &[u8] = b"get_name";
    pub const COMMON_VERSION_IDENT: &[u8] = b"get_version";
    pub const KIND_IDENT: &[u8] = b"get_kind";
    pub const DATA_IDENT: &[u8] = b"PLUGIN_DATA";
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal_define_plugin {
    (
        name: $name:literal,
        kind: $kind:literal,
        data_type: $data_type:ty,
        data: $data:expr
    ) => {
        // NOTE: in order to export data, it must be FFI-safe. This means that
        // we have to use types such as `*const c_char` rather than `&str`.
        // `CStr` is a wrapper and not `repr(C)`, so that won't work either.
        //
        // Furthermore, the exported data must be thread-safe (implement
        // `Sync`). Since `*const c_char` itself does not implement `Sync`
        // because it's a pointer, we have to use a function that returns the
        // string instead.

        const __NAME: &str = concat!($name, "\0");
        const __KIND: &str = concat!($kind, "\0");
        // For the version we use a string ("0.1.0") rather than a tuple of
        // integers such as `(0, 1, 0)`. Even though the latter works perfectly
        // for semantic versioning, it doesn't work with development versioning
        // ("master-2bb58f4") and more complex variations (we'll just keept it
        // simple for now).
        const __VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

        #[no_mangle]
        pub extern "C" fn get_name() -> *const ::std::os::raw::c_char {
            __NAME.as_ptr() as _
        }

        #[no_mangle]
        pub extern "C" fn get_kind() -> *const ::std::os::raw::c_char {
            __KIND.as_ptr() as _
        }

        #[no_mangle]
        pub extern "C" fn get_version() -> *const ::std::os::raw::c_char {
            __VERSION.as_ptr() as _
        }

        /// This should only be loaded after checking for version mismatches, so
        /// we may use custom types now.
        ///
        /// The `PluginData` type is thread-safe, so we don't need to use a
        /// function for this.
        #[no_mangle]
        pub static PLUGIN_DATA: $data_type = $data;
    };
}

/// This macro helps define a connector plugin consistently without the usual
/// boilerplate that is required.
#[macro_export]
macro_rules! define_connector_plugin {
    (
        name: $name:literal,
        data: $data:expr
    ) => {
        $crate::internal_define_plugin! {
            name: $name,
            kind: "connector",
            data_type: ConnectorPlugin,
            data: $data
        }
    };
}

/// This contains the data about the plugin
#[repr(C)]
#[derive(Clone)]
pub struct ConnectorPlugin {
    // Initializes the plugin by creating a new state
    //
    // The plugin should have a `destructor` function to manually drop the state
    // when it's being unloaded, but since we don't support plugin unloading
    // we'll skip it for now.
    pub new: unsafe extern "C" fn() -> *mut c_void,
    // A stub function exported by the connector
    pub something: unsafe extern "C" fn(state: *mut c_void) -> i32,
    // Other misc. metadata
    pub is_sink: bool,
    pub is_source: bool,
}
