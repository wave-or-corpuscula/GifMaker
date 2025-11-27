use std::{env, error::Error, fs::File, process::{Command, Stdio}, str::FromStr};

pub struct GifConfig {
    pub f_color: String,
    pub s_color: String,
    pub duration: u32,
    pub transition: String,
    pub font_size: u32,
    pub line_length: u32,
}

impl GifConfig {
    pub fn new(args: &[String]) -> Self {
        send_to_file(args[1].clone(), "f_text.txt");
        send_to_file(args[2].clone(), "s_text.txt");

        let f_color = parse_env("F_COLOR");
        let s_color = parse_env("S_COLOR");
        let duration = parse_env("DURATION");
        let transition = parse_env("TRANSITION");
        let font_size = parse_env("FONT_SIZE");
        let line_length = parse_env("LINE_LENGTH");

        Self {f_color, s_color, duration, transition, font_size, line_length}
    }
}

fn send_to_file(text: String, file_name: &str) -> Result<(), Box<dyn Error>> {
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

fn parse_env<T>(key: &str) -> T 
where
    T: FromStr
{
    env::var(key)
    .ok()
    .and_then(|var| var.parse::<T>().ok())
    .unwrap()
}