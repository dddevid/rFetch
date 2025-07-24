use thiserror::Error;

#[derive(Error, Debug)]
pub enum RFetchError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("System information error: {0}")]
    SystemInfo(String),

    #[error("Display error: {0}")]
    Display(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[cfg(unix)]
    #[error("Unix system error: {0}")]
    Unix(#[from] nix::Error),

    #[cfg(windows)]
    #[error("Windows system error: {0}")]
    Windows(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RFetchError {
    pub fn config<T: Into<String>>(msg: T) -> Self {
        RFetchError::Config(msg.into())
    }

    pub fn system_info<T: Into<String>>(msg: T) -> Self {
        RFetchError::SystemInfo(msg.into())
    }

    pub fn display<T: Into<String>>(msg: T) -> Self {
        RFetchError::Display(msg.into())
    }

    pub fn unknown<T: Into<String>>(msg: T) -> Self {
        RFetchError::Unknown(msg.into())
    }
}