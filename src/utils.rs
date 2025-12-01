use std::{env, fs::OpenOptions, io::Error, path::PathBuf, str::FromStr, io::Write};

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
        Err(_) => Ok(default)
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

pub fn get_file_abs_path(path: &str) -> Result<String, Error> {
    let srcdir = PathBuf::from(path);
    Ok(std::fs::canonicalize(&srcdir)?.display().to_string())
}

pub fn write_file(text: String, path: &str) -> Result<(), ConfigError>
{
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(ConfigError::IoError)?;

    file.write_all(&text.as_bytes())
        .map_err(ConfigError::IoError)
}