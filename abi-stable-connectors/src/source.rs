use abi_stable::std_types::RBox;
use std::time::Duration;
use tokio::{task, time};

use common_abi_stable_connectors::{
    source::{RawSource_TO, SourceContext, SourceReply},
    Result,
};

#[derive(Default)]
pub struct SourceAddr(String);

#[derive(Default)]
pub struct SourceManagerBuilder;
impl SourceManagerBuilder {
    pub fn spawn(
        self,
        source: RawSource_TO<'static, RBox<()>>,
        ctx: SourceContext,
    ) -> Result<SourceAddr> {
        let manager = SourceManager { source, ctx };
        // spawn manager task
        task::spawn(manager.run());

        Ok(SourceAddr::default())
    }
}

// The runner of the source
//
// Note that it uses `dyn` instead of generics now.
pub struct SourceManager {
    pub source: RawSource_TO<'static, RBox<()>>,
    pub ctx: SourceContext,
}
impl SourceManager {
    pub async fn run(mut self) -> Result<()> {
        loop {
            let data: Result<SourceReply> = self.source.pull_data(0, &self.ctx).into();
            match data? {
                SourceReply::Empty => {
                    println!("No data available")
                }
                SourceReply::Data(data) => {
                    println!("Sending '{}' to pipeline", data)
                }
                SourceReply::Sleep(ms) => {
                    time::sleep(Duration::from_millis(ms)).await;
                }
            }
        }
    }
}
