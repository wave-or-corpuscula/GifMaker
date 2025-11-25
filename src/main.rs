use ffmpeg_next as ffmpeg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Начинаем работу с ffmpeg...");

    // Инициализация ffmpeg
    ffmpeg::init()?;
    println!("FFmpeg инициализирован");

    // Для начала просто попробуем создать выходной файл
    test_file_creation()?;

    println!("Тест завершен!");
    Ok(())
}

fn test_file_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Создаем выходной файл...");

    // Создаем выходной файл
    let mut output = ffmpeg::format::output("test/output.mp4")?;
    println!("Файл создан");

    // Находим кодек
    let codec = ffmpeg::encoder::find(ffmpeg::codec::Id::H264);
    match codec {
        Some(c) => println!("Кодек H264 найден: {:?}", c.name()),
        None => println!("Кодек H264 не найден"),
    }

    // Пытаемся добавить поток
    match codec {
        Some(codec) => {
            let mut stream = output.add_stream(codec)?;
            println!("Поток добавлен");

            // Создаем контекст кодировщика как в примере
            let mut encoder = ffmpeg::codec::context::Context::new_with_codec(codec)
                .encoder()
                .video()?;

            // Устанавливаем параметры кодировщика
            encoder.set_width(1280);
            encoder.set_height(720);
            encoder.set_format(ffmpeg::format::Pixel::YUV420P);
            encoder.set_time_base(ffmpeg::Rational(1, 25));
            encoder.set_frame_rate(Some(ffmpeg::Rational(25, 1)));

            // Открываем кодировщик как в примере
            let opened_encoder = encoder.open_with(ffmpeg::Dictionary::new())?;
            println!("Кодировщик открыт");

            // Устанавливаем параметры в поток как в примере
            stream.set_parameters(&opened_encoder);
            println!("Параметры установлены");

            // Устанавливаем базовые параметры потока
            stream.set_time_base(ffmpeg::Rational(1, 25));
            stream.set_avg_frame_rate(ffmpeg::Rational(25, 1));
            println!("Time base установлен");

            // Для отладки посмотрим, что у нас в потоке
            println!("Индекс потока: {}", stream.index());
        }
        None => println!("Не удалось добавить поток")
    }

    // Записываем заголовок
    output.write_header()?;
    println!("Заголовок записан");

    // Завершаем
    output.write_trailer()?;
    println!("Trailer записан");

    Ok(())
}

