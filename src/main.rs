use image::{io::Reader, ColorType, GenericImageView};

mod qoi;
use qoi::{qoi_write, QOIHeader};

fn main() {
    let img = Reader::open("dice.png").unwrap().decode().unwrap();

    // Get information for header
    let width = img.width();
    let height = img.height();
    let channels = match img.color() {
        ColorType::Rgb8 => 3,
        ColorType::Rgba8 => 4,
        _ => 4,
    };
    let colorspace = 0_u8;

    let header = QOIHeader {
        width,
        height,
        channels,
        colorspace,
    };

    let data: Vec<[u8; 4]> = img.pixels().map(|p| p.2 .0).collect();
    let size = qoi_write("dice2.qoi", data, header);
    println!("{} bytes written", size);
}
