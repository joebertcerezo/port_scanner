use futures::stream::{self, StreamExt};

use crate::{
    config::Config,
    models::{PortState, ScanResult},
    scanner::target::scan_port,
};

pub struct ScanEngine {
    config: Config,
}

impl ScanEngine {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Vec<ScanResult> {
        let address = self.config.address;
        let req_timeout = self.config.timeout;
        let ports = self.config.start_port..=self.config.end_port;

        let mut results: Vec<ScanResult> = stream::iter(ports)
            .map(|port| async move { scan_port(address, port, req_timeout).await })
            .buffer_unordered(self.config.concurrency)
            .filter(|res| futures::future::ready(res.state == PortState::Open))
            .collect()
            .await;

        results.sort_unstable_by_key(|r| r.port);
        results
    }
}
