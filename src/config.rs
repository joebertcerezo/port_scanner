use std::net::IpAddr;

pub const MAX_CONCURRENCY_LIMIT: usize = 10_000;
pub const MIN_TIMEOUT_MS: u64 = 10;
pub const MAX_TIMEOUT_MS: u64 = 60_000;

pub struct Config {
  pub address: IpAddr
}
