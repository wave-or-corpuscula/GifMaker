use std::fmt::{Formatter, Result, Display};


#[derive(Debug)]
pub enum ConfigError {
    InvalidValue(String),
    IoError(std::io::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ConfigError::InvalidValue(val) => write!(f, "Wrong valud: {}", val),
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}


impl std::error::Error for ConfigError {}
