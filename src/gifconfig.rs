use std::{env, str::FromStr};

pub struct GifConfig {
    pub f_word: String,
    pub s_word: String,
    pub f_color: String,
    pub s_color: String,
    pub duration: u32,
    pub transition: String,
    pub font_size: u32,
    pub line_length: u32,
}

impl GifConfig {
    pub fn new(args: &[String]) -> Self {
        let f_word = args[1].clone();
        let s_word = args[2].clone();

        let f_color = parse_env("F_COLOR");
        let s_color = parse_env("S_COLOR");
        let duration = parse_env("DURATION");
        let transition = parse_env("TRANSITION");
        let font_size = parse_env("FONT_SIZE");
        let line_length = parse_env("LINE_LENGTH");

        Self {f_word, s_word, f_color, s_color, duration, transition, font_size, line_length}
    }
}

fn parse_env<T>(key: &str) -> T 
where
    T: FromStr
{
    env::var(key)
    .ok()
    .and_then(|var| var.parse::<T>().ok())
    .unwrap()
}