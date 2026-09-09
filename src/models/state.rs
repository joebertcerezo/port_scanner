use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    Error,
}

impl Display for PortState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortState::Open => write!(f, "OPEN"),
            PortState::Closed => write!(f, "CLOSED"),
            PortState::Filtered => write!(f, "FILTERED"),
            PortState::Error => write!(f, "ERROR"),
        }
    }
}
