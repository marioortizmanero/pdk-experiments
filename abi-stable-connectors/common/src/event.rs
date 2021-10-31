use crate::{Result, RResult, value::Value, DEFAULT_STREAM_ID};

use abi_stable::{StableAbi, std_types::{RBox, RVec, RString, RResult::ROk}};

#[repr(C)]
#[derive(StableAbi, Default)]
pub struct Event {
    /// The event ID
    pub id: i32,

    // The rest was simpified, as it's not really necessary for this example
}

// The event serializer is an opaque type because it's simpler than trying to
// make it `abi_stable`-compatible.
//
// Anyway, for this example it's quite simplified.
pub struct EventSerializer {
    postprocessor_names: Vec<String>,
}

// This is just the constructor, so it doesn't need to be in the trait for the
// opaque type.
impl EventSerializer {
    pub fn build(
        postprocessor_names: Vec<String>,
    ) -> Result<Self> {
        Ok(Self { postprocessor_names })
    }
}

#[abi_stable::sabi_trait]
pub trait RawEventSerializer {
    fn drop_stream(&mut self, _stream_id: u64) {
        unimplemented!();
    }

    /// clear out all streams - this can lead to data loss
    /// only use when you are sure, all the streams are gone
    fn clear(&mut self) {
        unimplemented!();
    }

    /// serialize event for the default stream
    ///
    /// # Errors
    ///   * if serialization failed (codec or postprocessors)
    fn serialize(&mut self, value: &Value, ingest_ns: u64) -> RResult<RVec<RVec<u8>>> {
        self.serialize_for_stream(value, ingest_ns, DEFAULT_STREAM_ID)
    }

    /// serialize event for a certain stream
    ///
    /// # Errors
    ///   * if serialization failed (codec or postprocessors)
    fn serialize_for_stream(
        &mut self,
        _value: &Value,
        _ingest_ns: u64,
        _stream_id: u64,
    ) -> RResult<RVec<RVec<u8>>> {
        // This should actually postprocess the value
        ROk(Default::default())
    }
}

impl RawEventSerializer for EventSerializer {}
pub type OpaqueEventSerializer = RawEventSerializer_TO<'static, RBox<()>>;

/// TODO: research usage of `Pin`
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct EventPayload {
    data: i32
}
