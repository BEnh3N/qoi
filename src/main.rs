use std::{env::args, process::exit};

use image::{io::Reader, GenericImageView};
use qoi::{qoi_write, QOIHeader, QOI_SRGB};

mod qoi;

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: qoi <infile> <outfile>");
        println!("Examples:");
        println!("   qoi input.png output.qoi");
        println!("   qoi input.qoi output.png");
        exit(1);
    }

    let pixels: Vec<[u8; 4]>;
    let w;
    let h;
    let channels;

    if args[1].ends_with(".png") {
        let img = Reader::open(&args[1]).unwrap().decode().unwrap();
        w = img.width();
        h = img.height();
        channels = 4;

        pixels = img.pixels().map(|x| x.2.0).collect();
    } else {
        println!("Invalid input file type!");
        exit(1);
    }

    // TODO: Implement reading from QOI files and writing to PNG files

    if args[2].ends_with(".qoi") {
        qoi_write(&args[2], pixels, QOIHeader {
            width: w,
            height: h,
            channels,
            colorspace: QOI_SRGB
        });
    }
}
