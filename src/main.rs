use std::{env, error::Error, fs::remove_file, io, process::{self, Command}};

use dotenv;

mod utils;
mod errors;
mod service;
mod gifconfig;
use gifconfig::GifConfig;
use utils::{split_by_lines, get_file_abs_path, write_file};
use service::{help_message, restore_config};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--first")) && args.contains(&String::from("--second")) {
        dotenv::dotenv().ok();
        let config = match GifConfig::parse() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error while config file parsing: {e}");
                process::exit(1);
            }
        };

        if let Err(e) = run_inline(args, config) {
            println!("Application error {e}");
            process::exit(1)
        }
        process::exit(0);
    }
    
    if args.contains(&String::from("--help")) {
        match help_message() {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("Error occured: {e}");
                process::exit(1)
            }
        };
    }

    if args.contains(&String::from("--restore-config")) {
        match restore_config() {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("Error occured: {e}");
                process::exit(1)
            }
        };
    }

    dotenv::dotenv().ok();

    let config = match GifConfig::parse() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error while config file parsing: {e}");
            process::exit(1);
        }
    };
    
    if let Err(e) = run(config) {
        println!("Application error {e}");
        process::exit(1)
    }
}

fn run_inline(args: Vec<String>, config: GifConfig) -> Result<(), Box<dyn Error>> {
    let mut first_phrase = String::new();
    let mut second_phrase = String::new();
    let mut collecting_first = false;
    let mut collecting_second = false;

    for arg in &args[1..] {
        if arg == "--first" {
            collecting_first = true;
            collecting_second = false;
        } else if arg == "--second" {
            collecting_first = false;
            collecting_second = true;
        } else if collecting_first {
            if !first_phrase.is_empty() {
                first_phrase.push(' ');
            }
            first_phrase.push_str(arg);
        } else if collecting_second {
            if !second_phrase.is_empty() {
                second_phrase.push(' ');
            }
            second_phrase.push_str(arg);
        }
    }

    if first_phrase.is_empty() || second_phrase.is_empty() {
        return Err("Both --first and --second phrases must be provided".into());
    }

    let f_text = split_by_lines(&first_phrase, config.line_length);
    let s_text = split_by_lines(&second_phrase, config.line_length);

    write_file(f_text, "/tmp/f_text.txt")
        .and_then(|_| write_file(s_text, "/tmp/s_text.txt"))?;

    create_background(&config)
        .and_then(|_| create_gif(&config))?;

    Ok(())
}



fn run(config: GifConfig) -> Result<(), Box<dyn Error>> {

    let (mut f_text, mut s_text) = (String::new(), String::new());

    println!("Enter first phrase:");
    io::stdin()
        .read_line(&mut f_text)
        .expect("Error while reading phrase.");
    
    println!("Enter second phrase:");
    io::stdin()
        .read_line(&mut s_text)
        .expect("Error while reading phrase.");

    f_text = split_by_lines(&f_text, config.line_length);
    s_text = split_by_lines(&s_text, config.line_length);
    
    write_file(f_text, "/tmp/f_text.txt")
        .and_then(|_| write_file(s_text, "/tmp/s_text.txt"))?;

    create_background(&config)
        .and_then(|_| create_gif(&config))?;
    Ok(())
}

fn create_background(config: &GifConfig) -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("ffmpeg")
    .args([
        "-y",
        "-loglevel", "error",
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
        "./gif/background.mp4"
        ])
    .spawn()
    .expect("cannot create background");
    child.wait()?;
    Ok(())
}

fn create_gif(config: &GifConfig) -> Result<(), Box<dyn Error>> {
    let gif_name = format!(
        "gif_{}.gif",
        chrono::Local::now().format("%Y-%m-%d_%H:%M:%S")
    );
    let mut child = Command::new("ffmpeg")
    .args([
        "-y",
        "-loglevel", "error",
        "-i", "./gif/background.mp4",
        "-vf", &format!("drawtext=textfile=/tmp/f_text.txt:reload=1:text_align=center:line_spacing=-10:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={},\
            drawtext=textfile=/tmp/s_text.txt:text_align=center:x=(w-text_w)/2:y=(h-text_h)/2:fontsize={}:fontcolor={}:alpha='if(gte(t,2),if(lte(t,4),(t-2)/2,1),0)'",
        config.font_size,
        config.s_color,
        config.font_size,
        config.f_color),
        "-c:a", "copy", &format!("./gif/{}", gif_name)
    ])
    .spawn()
    .expect("cannot ");
    child.wait()?;
    remove_file("./gif/background.mp4")?;
    let output_path = get_file_abs_path(&format!("./gif/{}", gif_name))?;
    println!("Your file saved at: {}", output_path);
    Command::new("xdg-open").arg(&output_path).spawn()?;
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