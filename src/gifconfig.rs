use std::{env, error::Error, fs::{File, OpenOptions}, io::Write, process::{Command, Stdio}, str::FromStr};

use crate::utils::parse_env_with_default;
use crate::errors::ConfigError;


pub struct GifConfig {
    pub f_color: String,
    pub s_color: String,
    pub duration: u32,
    pub transition: String,
    pub font_size: u32,
    pub line_length: u32,
}

impl GifConfig {
    pub fn parse() -> Result<Self, ConfigError> {
        let f_color = parse_env_with_default("F_COLOR", "RED".to_string())?;
        let s_color = parse_env_with_default("S_COLOR", "BLUE".to_string())?;
        let duration = parse_env_with_default("DURATION", 5)?;
        let transition = parse_env_with_default("TRANSITION", "vertclose".to_string())?;
        let font_size = parse_env_with_default("FONT_SIZE", 20)?;
        let line_length = parse_env_with_default("LINE_LENGTH", 15)?;

        Ok(Self {f_color, s_color, duration, transition, font_size, line_length})
    }
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

pub fn send_to_file(text: String, file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(&format!("/tmp/{}", file_name))?;
    let mut child = Command::new("echo")
    .args([
        "-e",
        &format!("{}", text),
    ])
    .stdout(Stdio::from(file))
    .spawn()
    .expect(&format!("cannot write text: {} in file: {}", text, file_name));
     child.wait()?;
     Ok(())
}
