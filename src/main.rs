use std::{
    env::args,
    fs::File,
    process::{exit, ExitCode},
};

use image::{codecs::png::PngEncoder, io::Reader, ColorType, ImageEncoder};
use qoi::{qoi_write, QOIHeader, QOI_SRGB};

mod qoi;

fn main() -> ExitCode {
    let args = args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: qoi <infile> <outfile>");
        println!("Examples:");
        println!("   qoi input.png output.qoi");
        println!("   qoi input.qoi output.png");
        exit(1);
    }

    let pixels: Vec<u8>;
    let w;
    let h;
    let channels;

    if args[1].ends_with(".png") {
        let img = Reader::open(&args[1]).unwrap().decode().unwrap();
        w = img.width();
        h = img.height();
        channels = match img.color() {
            ColorType::Rgb8 => 3,
            ColorType::Rgba8 => 4,
            _ => 4,
        };

        pixels = img.as_bytes().to_vec();
    } else {
        eprintln!("Invalid input file type!");
        return ExitCode::FAILURE;
    }
    // TODO: Implement reading from QOI files

    // TODO: is this really the best way to encode pngs... it's ugly
    if args[2].ends_with(".png") {
        let encoder = PngEncoder::new(File::create(&args[2]).unwrap());
        let color_type = match channels {
            3 => ColorType::Rgb8,
            _ => ColorType::Rgba8,
        };
        encoder.write_image(&pixels, w, h, color_type).unwrap();
    } else if args[2].ends_with(".qoi") {
        qoi_write(
            &args[2],
            pixels,
            QOIHeader {
                width: w,
                height: h,
                channels,
                colorspace: QOI_SRGB,
            },
        );
    }

    ExitCode::SUCCESS
}
