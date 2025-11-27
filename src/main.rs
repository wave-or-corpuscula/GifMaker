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
    let child = Command::new("echo")
    .args([
        "hello"
        ])
    .spawn()
    .expect("(((");
    Ok(())
}

fn create_background(config: GifConfig) -> Result<(), Box<dyn Error>> {
    let child = Command::new("ffmpeg")
    .args([
        "-y",
        "-filter_complex",
        ])
    .spawn()
    .expect("(((");
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