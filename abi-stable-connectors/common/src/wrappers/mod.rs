use abi_stable::{
    std_types::{RBox, RString, ROption},
    StableAbi,
};
use cglue::cglue_trait;

/// Internal type with types that aren't wrapped by `abi_stable`
#[repr(C)]
#[derive(StableAbi)]
pub struct ConnectorContext {
    /// unique identifier
    pub uid: u64,
    /// url of the connector
    pub url: RString,
    /// type name of the connector
    pub type_name: RString,
    /// oh no! there's no `serde_yaml::Value` in `abi_stable`, we can't
    /// `#[derive(StableAbi)]`!
    ///
    /// Solution: using its opaque alternative
    pub enabled: Value_TO<'static, RBox<()>>,
}

#[abi_stable::sabi_trait]
pub trait Value {
    fn as_bool(&self) -> ROption<bool>;
    fn as_i64(&self) -> ROption<i64>;
    fn as_null(&self) -> ROption<()>;
}

impl Value for serde_yaml::Value {
    fn as_bool(&self) -> ROption<bool> {
        self.as_bool().into()
    }

    fn as_i64(&self) -> ROption<i64> {
        self.as_i64().into()
    }

    fn as_null(&self) -> ROption<()> {
        self.as_null().into()
    }
}
