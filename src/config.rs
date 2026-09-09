use std::{net::IpAddr, time::Duration};

use crate::error::ConfigError;

pub const MAX_CONCURRENCY_LIMIT: usize = 10_000;
pub const MIN_TIMEOUT_MS: u64 = 10;
pub const MAX_TIMEOUT_MS: u64 = 60_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub address: IpAddr,
    pub start_port: u16,
    pub end_port: u16,
    pub concurrency: usize,
    pub timeout: Duration,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.start_port == 0 || self.end_port == 0 {
            return Err(ConfigError::PortZeroNotSupported);
        }
        if self.start_port > self.end_port {
            return Err(ConfigError::InvalidPortRange {
                start: self.start_port,
                end: self.end_port,
            });
        }
        if self.concurrency == 0 || self.concurrency > MAX_CONCURRENCY_LIMIT {
            return Err(ConfigError::ConcurrencyOutOfBounds {
                requested: self.concurrency,
                max: MAX_CONCURRENCY_LIMIT,
            });
        }
        let timeout_ms = self.timeout.as_millis() as u64;
        // if timeout_ms < MIN_TIMEOUT_MS || timeout_ms > MAX_TIMEOUT_MS {
        if !(MIN_TIMEOUT_MS..MAX_TIMEOUT_MS).contains(&timeout_ms) {
            // if matches!(timeout_ms, MIN_TIMEOUT_MS..=MAX_TIMEOUT_MS) {
            return Err(ConfigError::TimeoutOutOfBounds {
                requested_ms: timeout_ms,
                min_ms: MIN_TIMEOUT_MS,
                max_ms: MAX_TIMEOUT_MS,
            });
        }
        Ok(())
    }
}
