use std::net::IpAddr;

use crate::{models::state::PortState, scanner::service::Service};

pub struct ScanResult {
    pub address: IpAddr,
    pub port: u16,
    pub state: PortState,
    pub service: Service,
}

impl ScanResult {
    fn new(address: IpAddr, port: u16, state: PortState, service: Service) -> Self {
        Self { address, port, state, service }
    }
}
