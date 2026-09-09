use std::net::{IpAddr, Ipv4Addr};

use bpaf::Bpaf;

const DEFAULT_LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_CONCURRENCY: usize = 100;
const DEFAULT_TIMEOUT_MS: u64 = 1500;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct CliArgs {
    #[bpaf(long("address"), short('a'), argument("ADDRESS"), fallback(DEFAULT_LOCALHOST))]
    /// Target IPv4 or IPv6 address [default: 127.0.0.1]
    pub address: IpAddr,

    #[bpaf(long("start"), short('s'), fallback(1u16))]
    /// Starting port number [default: 1]
    pub start_port: u16,

    #[bpaf(long("end"), short('e'), fallback(u16::MAX))]
    /// Ending port number [default: 65535]
    pub end_port: u16,

    #[bpaf(long("concurrency"), short('c'), fallback(DEFAULT_CONCURRENCY))]
    /// Maximum concurrent socket connections [default: 100]
    pub concurrency: usize,

    #[bpaf(long("timeout"), short('t'), fallback(DEFAULT_TIMEOUT_MS))]
    /// Connection timeout in milliseconds [default: 1500]
    pub timeout_ms: u64,
}
