use std::{fs::File, io::Write};

use image::{io::Reader, ColorType, GenericImageView};

struct QOIHeader {
    magic: [char; 4], // magic bytes "qoif"
    width: u32,       // image width in pixels (BE)
    height: u32,      // image height in pixels (BE)
    channels: u8,     // 3 = RGB, 4 = RGBA
    colorspace: u8    // 0 = sRGB with linear alpha
                      // 1 = all channels linear
}

fn main() {
    let img = Reader::open("input.png").unwrap().decode().unwrap();

    // Get information for header
    let magic = ['q', 'o', 'i', 'f'];
    let width = img.width();
    let height = img.height();
    let channels = match img.color() {
        ColorType::Rgb8 => 3,
        ColorType::Rgba8 => 4,
        _ => 4,
    };
    let colorspace = 0_u8;

    let header = QOIHeader {
        magic,
        width,
        height,
        channels,
        colorspace
    };

    let mut qoi = File::create("output.qoi").unwrap();
    write_header(&mut qoi, &header);

    let mut prev_pixels = [0; 64];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;

            // println!("{:?}", pixel.0);
        }
    }
}

fn write_header(file: &mut File, header: &QOIHeader) {
    file.write_all("qoif".as_bytes()).unwrap();
    file.write_all(&header.width.to_be_bytes()).unwrap();
    file.write_all(&header.height.to_be_bytes()).unwrap();
    file.write_all(&header.channels.to_be_bytes()).unwrap();
    file.write_all(&header.colorspace.to_be_bytes()).unwrap();
}
