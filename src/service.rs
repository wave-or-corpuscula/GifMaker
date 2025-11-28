use std::error::Error;

use crate::utils::{get_file_abs_path, write_file};

pub fn help_message() -> Result<(), Box<dyn Error>>{
    let env_path = get_file_abs_path("./.env").unwrap_or(String::from("Не удалось найти файл конфигурации"));
    println!("\
Использование команды:
gifmaker 
Введите первую фразу: [вводим]
Введите вторую фразу: [вводим]

Конфигурации можно изменить, отредактировав файл с конфигурациями:
{env_path}

--restore-config    Восстановить файл конфигурации 
");

    Ok(())
}

pub fn restore_config() -> Result<(), Box<dyn Error>> {
    let config_text = "\
# All avaliable colors
# ffmpeg -colors
F_COLOR=DarkGreen
S_COLOR=DarkBlue

# All avaliable transitions
# ffmpeg --help filter=xfade
TRANSITION=vertclose

DURATION=5
FONT_SIZE=20
# Max characters amount in one line
LINE_LENGTH=20";

    write_file(String::from(config_text), ".env")?;
    println!("Файл конфигурации восстановлен!");
    Ok(())
}