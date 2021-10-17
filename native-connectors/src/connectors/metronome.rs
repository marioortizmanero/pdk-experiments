// Copyright 2021, The Tremor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::{connectors::prelude::*, time::nanotime, DEFAULT_STREAM_ID};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Metronome {
    interval: u64,
    next: u64,
    origin_uri: EventOriginUri,
}

/// NOTE: simplification of the real type. `ConnectorBuilder` was removed as a
/// trait.
#[derive(Debug, Default)]
pub(crate) struct Builder {}
impl Builder {
    pub fn from_config(
        &self,
        _id: &TremorUrl,
        interval: u64, // NOTE: simpler than `OpConfig`
    ) -> Box<dyn Connector> {
        let origin_uri = EventOriginUri {
            scheme: "tremor-metronome".to_string(),
            host: hostname(),
            port: None,
            path: vec![interval.to_string()],
        };

        Box::new(Metronome {
            interval,
            next: 0,
            origin_uri,
        })
    }
}

impl Source for Metronome {
    fn pull_data(&mut self, pull_id: u64, _ctx: &SourceContext) -> Result<SourceReply> {
        let now = nanotime();
        if self.next < now {
            self.next = now + self.interval;
            // NOTE: removed the `literal` macro
            let mut data = HashMap::new();
            data.insert("onramp".to_string(), "metronome".to_string());
            data.insert("ingest_ns".to_string(), now.to_string());
            data.insert("id".to_string(), pull_id.to_string());
            Ok(SourceReply::Structured {
                origin_uri: self.origin_uri.clone(),
                payload: data,
                stream: DEFAULT_STREAM_ID,
            })
        } else {
            Ok(SourceReply::Empty(self.next - now))
        }
    }

    fn is_transactional(&self) -> bool {
        false
    }
}
impl Connector for Metronome {
    /* async */
    fn connect(
        &mut self,
        _ctx: &ConnectorContext,
        _notifier: super::reconnect::ConnectionLostNotifier,
    ) -> Result<bool> {
        Ok(true)
    }

    /* async */
    fn on_start(&mut self, _ctx: &ConnectorContext) -> Result<ConnectorState> {
        Ok(ConnectorState::Running)
    }

    fn default_codec(&self) -> &str {
        "json"
    }

    /* async */
    fn create_source(
        &mut self,
        source_context: SourceContext,
        builder: super::source::SourceManagerBuilder,
    ) -> Result<Option<super::source::SourceAddr>> {
        builder.spawn(self.clone(), source_context).map(Some)
    }
}
