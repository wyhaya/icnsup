mod app;

use icns::{IconFamily, Image, PixelFormat};
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView};
use std::fs::File;

macro_rules! exit {
    ($($arg:tt)*) => {
        {
            eprint!("Error: ");
            eprintln!($($arg)*);
            std::process::exit(1)
        }
    };
}

fn main() {
    let (input, output) = app::options();

    let source = match ImageReader::open(input) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(err) => exit!("{:?}", err),
        },
        Err(err) => exit!("{:?}", err),
    };

    if source.height() < 1024 || source.width() < 1024 || source.height() != source.width() {
        exit!("Please select a square image larger than 1024 in width and height");
    }

    let f = File::create(&output).unwrap_or_else(|err| exit!("{:?}", err));

    let mut icns = IconFamily::new();

    let sizes = vec![16, 32, 48, 64, 128, 256, 512, 1024];
    for size in sizes {
        let bytes = source
            .resize(size, size, FilterType::Triangle)
            .to_rgba8()
            .to_vec();
        let icon = Image::from_data(PixelFormat::RGBA, size, size, bytes)
            .unwrap_or_else(|err| exit!("{:?}", err));
        icns.add_icon(&icon)
            .unwrap_or_else(|err| exit!("{:?}", err));
    }

    icns.write(f).unwrap_or_else(|err| exit!("{:?}", err));

    println!("Complete: {}", output.display());
}
