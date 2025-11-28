use std::{env, error::Error, io, process::{self, Command}};

use dotenv;

mod utils;
mod errors;
mod gifconfig;
use gifconfig::{GifConfig, write_file};
use crate::{errors::ConfigError};
use utils::split_by_lines;

fn main() {
    dotenv::dotenv().ok();

    let config = match GifConfig::parse() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Ошибка во время парсинга файла конфигураций: {e}");
            process::exit(1);
        }
    };
    
    if let Err(e) = run(config) {
        println!("Ошибка приложения {e}");
        process::exit(1)
    }
}

fn run(config: GifConfig) -> Result<(), Box<dyn Error>> {

    let (mut f_text, mut s_text) = (String::new(), String::new());
    // let (mut f_text, mut s_text) = (
    //     String::from("Достаточно длинная фраза, чтобы ее перенести"), 
    //     String::from("Еще одна, не менее длинная фраза, ага ага ага")
    // );

    println!("Введите первую фразу:");
    io::stdin()
        .read_line(&mut f_text)
        .expect("Ошибка при чтении фразы");
    
    println!("Введите вторую фразу:");
    io::stdin()
        .read_line(&mut s_text)
        .expect("Ошибка при чтении фразы");

    f_text = split_by_lines(&f_text, config.line_length);
    s_text = split_by_lines(&s_text, config.line_length);
    
    write_file(f_text, "/tmp/f_text.txt")?;
    write_file(s_text, "/tmp/s_text.txt")?;

    create_background(&config)
        .and_then(|_| create_gif(&config))?;
    Ok(())
}

fn create_background(config: &GifConfig) -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("ffmpeg")
    .args([
        "-y",
        "-filter_complex",
        &format!("color=c={}:d={}s [f_color]; \
        color=c={}:d={}s [s_color]; \
        [f_color][s_color]xfade=transition={}\
            :duration={}s", 
            config.f_color,
            config.duration,
            config.s_color,
            config.duration,
            config.transition,
            config.duration,
        ),
        "./test/background.mp4"
        ])
    .spawn()
    .expect("could not create a background");
    println!("Background created, waiting for ffmpeg to finish");
    child.wait()?;
    println!("ffmpeg finished");
    Ok(())
}

fn create_gif(config: &GifConfig) -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("ffmpeg")
    .args([
        "-y", "-i", "./test/background.mp4",
        "-vf", &format!("drawtext=textfile=/tmp/f_text.txt:reload=1:text_align=center:line_spacing=-10:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={},\
            drawtext=textfile=/tmp/s_text.txt:text_align=center:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={}:alpha='if(gte(t,2),if(lte(t,4),(t-2)/2,1),0)'",
        config.font_size,
        config.s_color,
        config.font_size,
        config.f_color),
        "-c:a", "copy", "./test/output.gif"
    ])
    .spawn()
    .expect("cannot cover background with text");
    child.wait()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dotenv_loading() {
        dotenv::dotenv().ok();
        let _config = GifConfig::parse();
    }
}