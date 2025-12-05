use std::error::Error;

use crate::utils::{get_file_abs_path, write_file};

pub fn help_message() -> Result<(), Box<dyn Error>>{
    let env_path = get_file_abs_path("./.env").unwrap_or(String::from("Cannot find configuration file"));
    println!("\
Usage:

Interactive mode:
gifmaker
Enter first phrase: [enter]
Enter second phrase: [enter]

Inline mode:
gifmaker --first \"Your first phrase\" --second \"Your second phrase\"

You can change configuration at configuration file:
{env_path}

--restore-config    Restore configuration file
");

    Ok(())
}

pub fn restore_config() -> Result<(), Box<dyn Error>> {
    let config_text = "\
# All available colors
# ffmpeg -colors
F_COLOR=DarkGreen
S_COLOR=DarkBlue

# All available transitions
# ffmpeg --help filter=xfade
TRANSITION=vertclose

DURATION=5
FONT_SIZE=20
# Max characters amount in one line
LINE_LENGTH=20";

    write_file(String::from(config_text), ".env")?;
    println!("Configuration file restored!");
    Ok(())
}