use std::fmt::{Formatter, Result, Display};


#[derive(Debug)]
pub enum ConfigError {
    MissingVar(String),
    InvalidValue(String),
    IoError(std::io::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ConfigError::MissingVar(key) => write!(f, "Отсутствует переменная окружения: {}", key),
            ConfigError::InvalidValue(val) => write!(f, "Неверное значение: {}", val),
            ConfigError::IoError(e) => write!(f, "Ошибка ввода/вывода: {}", e),
        }
    }
}


impl std::error::Error for ConfigError {}
