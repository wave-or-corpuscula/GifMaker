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
    let _ = create_gif(&config);
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
        "-vf", &format!("drawtext=textfile=/tmp/f_text.txt:reload=1:line_spacing=-10:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={},\
            drawtext=textfile=/tmp/s_text.txt:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={}:alpha='if(gte(t,2),if(lte(t,4),(t-2)/2,1),0)'",
        config.font_size,
        config.s_color,
        config.font_size,
        config.f_color),
        "-c:a", "copy", "./test/output.gif"
    ])
    .spawn()
    .expect("cannot cover background with text");
    child.wait();
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