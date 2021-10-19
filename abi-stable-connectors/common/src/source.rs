use abi_stable::{StableAbi, std_types::RString};
use std::time::Duration;
use tokio::{task, sync::mpsc::unbounded_channel};

use crate::connectors::{RResult, Result};

// Stubs for the original trait. We can't use `()` because it's not FFI-safe.
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SourceAddr(RString);
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SourceContext(RString);

#[repr(C)]
#[derive(StableAbi)]
pub enum SourceReply {
    Empty,
    Sleep(i32),
    Data(String), // should be a vector of u8 or similars
}

#[async_trait::async_trait]
pub trait Source: Send {
    async fn pull_data(&mut self, pull_id: u64, ctx: &SourceContext) -> Result<SourceReply>;
}

#[allow(clippy::module_name_repetitions)]
#[repr(C)]
#[derive(StableAbi)]
pub struct SourceManagerBuilder;
impl SourceManagerBuilder {
    pub fn spawn<S>(self, source: S, ctx: SourceContext) -> Result<SourceAddr>
    where
        S: Source + Send + 'static,
    {
        let manager = SourceManager(source);
        // spawn manager task
        task::Builder::new().name(name).spawn(manager.run())?;

        Ok(SourceAddr)
    }
}

// The runner of the source
pub struct SourceManager<S: Source> {
    pub source: S
}
impl<S: Source> SourceManager<S> {
    pub async fn run(&self) -> Result<()> {
        loop {
            match self.source.pull_data().await? {
                SourceReply::Sleep(ms) => task::sleep(Duration::from_millis(ms)),
                SourceReply::Data(data) => println!("Sending '{}' to pipeline", data),
            }
        }
    }
}
