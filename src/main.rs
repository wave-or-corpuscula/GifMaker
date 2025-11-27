use std::{env, error::Error, process::{self, Command}};

use dotenv;
mod gifconfig;
use gifconfig::GifConfig;
fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments!")
    }

    let config = GifConfig::new(&args);
    
    if let Err(e) = run(config) {
        println!("Application error {e}");
        process::exit(1)
    }
}
fn run(config: GifConfig) -> Result<(), Box<dyn Error>> {
    
    let _ = create_background(&config);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dotenv_loading() {
        dotenv::dotenv().ok();
        let config = GifConfig::new(&[String::new(), 
                                                    String::new(), 
                                                    String::new()]);
        println!("F_COLOR={}", config.f_color);
        println!("S_COLOR={}", config.s_color);
        println!("DURATION={}", config.duration);
        println!("TRANSITION={}", config.transition);
        println!("FONT_SIZE={}", config.font_size);
        println!("LINE_LENGTH={}", config.line_length);
    }
}