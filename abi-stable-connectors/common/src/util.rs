use abi_stable::StableAbi;

/// Panic errors should be treated differently from a regular error. Apart from
/// the fact that `panic::catch_unwind` returns a `Box<dyn Any>`, which is not
/// compatible with a `Box<dyn Error>`, panics happen when the plugin reaches an
/// unrecoverable state and cannot continue.
///
/// Instead of using `RResult` for panics, this is a different type that only
/// returns the original type if the function finished without panicking. Since
/// the contents returned by `catch_unwind` are just `dyn Any` and don't provide
/// any value, they're discarded. The panicking information will be printed
/// automatically as output.
///
/// This is used in FFI contexts, so it also implements `StableAbi`.
//
// Ideally, this should be accompanied with a `#[may_panic]` proc macro that
// adds the return type automatically and the `catch_unwind` to the function
// it's attached to. Additionally, it could come with a `#[may_not_panic]` proc
// macro as well that attaches the `#[no_panic]` macro to make sure the
// statement is true (https://github.com/dtolnay/no-panic). That crate isn't too
// reliable, so perhaps it could be opt-in (`#[may_not_panic(enforce)]`).
#[repr(C)]
#[derive(Debug, StableAbi)]
pub enum MayPanic<T> {
    Panic,
    NoPanic(T)
}

impl<T> MayPanic<T> {
    /// NOTE: Until https://doc.rust-lang.org/std/ops/trait.Try.html is
    /// stabilized.
    pub fn unwrap(self) -> T {
        match self {
            MayPanic::Panic => panic!("unwrap: unhandled panic"),
            MayPanic::NoPanic(t) => t
        }
    }
}

impl<T: Default> Default for MayPanic<T> {
    fn default() -> Self {
        MayPanic::NoPanic(T::default())
    }
}

/// For `catch_unwind` mostly
impl<T> From<std::thread::Result<T>> for MayPanic<T> {
    fn from(res: std::thread::Result<T>) -> Self {
        match res {
            Ok(val) => MayPanic::NoPanic(val),
            Err(_) => MayPanic::Panic
        }
    }
}
