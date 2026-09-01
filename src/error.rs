use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum ConfigError {
    InvalidPortRange { start: u16, end: u16 },
    PortZeroNotSupported,
    ConcurrencyOutOfBounds { requested: usize, max: usize },
    TimeoutOutOfBounds { requested_ms: u64, min_ms: u64, max_ms: u64 },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ConfigError::InvalidPortRange { start, end } => {
                write!(f, "start_port ({start}) cannot be greater than end_port ({end})")
            }
            ConfigError::PortZeroNotSupported => {
                write!(f, "port 0 is not supported as a scan target")
            }
            ConfigError::ConcurrencyOutOfBounds { requested, max } => {
                write!(f, "concurrency ({requested}) must be between 1 and {max}")
            }
            ConfigError::TimeoutOutOfBounds { requested_ms, min_ms, max_ms } => {
                write!(f, "timeout ({requested_ms}ms) must be between {min_ms}ms and {max_ms}ms")
            }
        }
    }
}

impl Error for ConfigError {}

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Io(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppError::Config(err) => write!(f, "Configuration error: {err}"),
            AppError::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        AppError::Config(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}
