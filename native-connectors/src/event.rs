/// NOTE: simplification of the real type
pub struct EventPayload;

/// NOTE: simplification of the real type
pub struct Event;

/// NOTE: simplification of the real type
pub struct EventSerializer;

/// Event origin URI
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventOriginUri {
    /// schema part
    pub scheme: String,
    /// host part
    pub host: String,
    /// port part
    pub port: Option<u16>,
    /// path part
    pub path: Vec<String>,
    // implement query params if we find a good usecase for it
    //pub query: Hashmap<String, String>
}
