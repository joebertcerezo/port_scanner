use port_scanner::{app::App, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    App::run().await
}

#[cfg(test)]
mod tests {
    use port_scanner::error::ConfigError;
    use port_scanner::{config, models, scanner};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::time::Duration;
    use tokio::net::TcpListener;

    use config::{Config, MAX_CONCURRENCY_LIMIT};
    use models::PortState;
    use scanner::service::Service;
    use scanner::target::scan_port;

    #[test]
    fn test_config_validation_boundaries() {
        let base = Config {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            start_port: 1,
            end_port: 100,
            concurrency: 50,
            timeout: Duration::from_millis(1000),
        };

        assert!(base.validate().is_ok());

        assert_eq!(
            Config { start_port: 0, ..base.clone() }.validate(),
            Err(ConfigError::PortZeroNotSupported)
        );

        assert_eq!(
            Config { start_port: 200, end_port: 100, ..base.clone() }.validate(),
            Err(ConfigError::InvalidPortRange { start: 200, end: 100 })
        );

        assert_eq!(
            Config { concurrency: 0, ..base.clone() }.validate(),
            Err(ConfigError::ConcurrencyOutOfBounds { requested: 0, max: MAX_CONCURRENCY_LIMIT })
        );

        assert_eq!(
            Config { concurrency: MAX_CONCURRENCY_LIMIT + 1, ..base.clone() }.validate(),
            Err(ConfigError::ConcurrencyOutOfBounds {
                requested: MAX_CONCURRENCY_LIMIT + 1,
                max: MAX_CONCURRENCY_LIMIT
            })
        );
    }

    #[tokio::test]
    async fn test_ephemeral_tcp_listener_scan() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let bound_port = listener.local_addr()?.port();

        let address = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let res = scan_port(address, bound_port, Duration::from_millis(500)).await;

        assert_eq!(res.state, PortState::Open);
        assert_eq!(res.port, bound_port);
        Ok(())
    }

    #[tokio::test]
    async fn test_closed_port_detection() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let bound_port = listener.local_addr()?.port();
        drop(listener);

        let address = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let res = scan_port(address, bound_port, Duration::from_millis(500)).await;

        assert_eq!(res.state, PortState::Closed);
        Ok(())
    }

    #[tokio::test]
    async fn test_ipv6_local_scan() {
        let Ok(listener) = TcpListener::bind("[::1]:0").await else {
            return;
        };
        let Ok(bound_port_check) = listener.local_addr() else {
            return;
        };
        let bound_port = bound_port_check.port();

        let address = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let res = scan_port(address, bound_port, Duration::from_millis(500)).await;

        assert_eq!(res.state, PortState::Open);
        assert_eq!(res.address, address);
    }

    #[test]
    fn test_service_enum_mappings() {
        assert_eq!(Service::from_port(80), Service::Http);
        assert_eq!(Service::from_port(443), Service::Https);
        assert_eq!(Service::from_port(6379), Service::Redis);
        assert_eq!(Service::from_port(55555), Service::Unknown);
    }
}
