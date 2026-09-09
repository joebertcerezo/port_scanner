use std::{
    io::ErrorKind,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use tokio::{net::TcpStream, time::timeout};

use crate::{
    models::{PortState, ScanResult},
    scanner::Service,
};

pub async fn scan_port(address: IpAddr, port: u16, request_timeout: Duration) -> ScanResult {
    let target = SocketAddr::new(address, port);
    let service = Service::from_port(port);

    match timeout(request_timeout, TcpStream::connect(target)).await {
        Ok(Ok(_stream)) => ScanResult::new(address, port, PortState::Open, service),
        Ok(Err(io_err)) => match io_err.kind() {
            ErrorKind::ConnectionRefused => {
                ScanResult::new(address, port, PortState::Closed, service)
            }
            _ => ScanResult::new(address, port, PortState::Error, service),
        },
        Err(_elapsed) => ScanResult::new(address, port, PortState::Filtered, service),
    }
}
