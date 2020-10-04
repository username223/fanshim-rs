use std::fmt::Formatter;
#[derive(Debug)]
pub enum FanshimError {
    IoError,
    ConfigParseError,
    SetLoggerError,
}

impl std::error::Error for FanshimError {}

impl std::fmt::Display for FanshimError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FanshimError::IoError => write!(f, "IoError"),
            FanshimError::ConfigParseError => write!(f, "ConfigParseError"),
            FanshimError::SetLoggerError => write!(f, "SetLoggerError"),
        }
    }
}

impl From<toml::de::Error> for FanshimError {
    fn from(_: toml::de::Error) -> Self {
        FanshimError::ConfigParseError
    }
}

impl From<std::io::Error> for FanshimError {
    fn from(_: std::io::Error) -> Self {
        FanshimError::IoError
    }
}

impl From<log::SetLoggerError> for FanshimError {
    fn from(_: log::SetLoggerError) -> Self {
        FanshimError::SetLoggerError
    }
}
