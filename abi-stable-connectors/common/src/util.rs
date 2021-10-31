use abi_stable::StableAbi;

/// Panic errors should be treated differently from a regular error. Apart from
/// the fact that `panic::catch_unwind` returns a `Box<dyn Any>`, which is not
/// compatible with a `Box<dyn Error>`, panics happen when the plugin reaches an
/// unrecoverable state and cannot continue.
///
/// Instead of using `RResult` for panics, this is a different type that only
/// returns the original type if the function finished without panicking. Since
/// the contents returned by `catch_unwind` are just `dyn Any` and don't provide
/// much value, they're discarded.
///
/// Hopefully we will be able to remove this workaround as soon as the
/// `C-unwind`:
///
/// https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
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

/// For conversions from `catch_unwind` mostly
impl<T> From<std::thread::Result<T>> for MayPanic<T> {
    fn from(res: std::thread::Result<T>) -> Self {
        match res {
            Ok(val) => MayPanic::NoPanic(val),
            Err(_) => MayPanic::Panic
        }
    }
}
