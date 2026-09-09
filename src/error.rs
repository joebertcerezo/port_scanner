use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("start_port ({start}) cannot be greater than end_port ({end})")]
    InvalidPortRange { start: u16, end: u16 },

    #[error("port 0 is not supported as a scan target")]
    PortZeroNotSupported,

    #[error("concurrency ({requested}) must be between 1 and {max}")]
    ConcurrencyOutOfBounds { requested: usize, max: usize },

    #[error("timeout ({requested_ms}ms) must be between {min_ms}ms and {max_ms}ms")]
    TimeoutOutOfBounds { requested_ms: u64, min_ms: u64, max_ms: u64 },
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
