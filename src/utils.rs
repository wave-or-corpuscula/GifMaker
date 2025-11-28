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

pub fn split_by_lines(text: &str, line_length: u32) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut result = String::new();
    let mut current_line_length = 0;

    for word in words {
        if current_line_length > 0 &&
           current_line_length + word.chars().count() > line_length as usize {
            result.push_str(&format!("\n{}", word));
            current_line_length = word.chars().count();
        } else if current_line_length > 0 {
            result.push_str(&format!(" {}", word));
            current_line_length += word.chars().count();
        } else {
            result.push_str(word);
            current_line_length = word.chars().count();
        }
    }

    result
}
