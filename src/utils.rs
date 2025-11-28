use std::{env, str::FromStr};

use crate::errors::ConfigError;

pub fn parse_env_with_default<T>(key: &str, default: T) -> Result<T, ConfigError>
where
    T: FromStr + Clone,
{
    match env::var(key) {
        Ok(var) => {
            var.parse()
                .map_err(|_| ConfigError::InvalidValue(var))
        }
        Err(_) => Ok(default) // Если переменная отсутствует - возвращаем значение по умолчанию
    }
}

fn parse_env_required<T>(key: &str) -> Result<T, ConfigError>
where
    T: FromStr,
{
    match env::var(key) {
        Ok(var) => {
            var.parse()
                .map_err(|_| ConfigError::InvalidValue(var))
        }
        Err(_) => Err(ConfigError::MissingVar(key.to_string()))
    }
}

pub fn split_by_lines(text: String, line_length: u32) -> String {
    let mut new_text = String::from(text);
    let mut index: usize = line_length as usize - 1;
    while new_text.len() > index{
        new_text.insert_str(index, "\n");
        index += line_length as usize;
    }
    new_text
}
