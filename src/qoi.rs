use std::{fs::File, io::Write, mem::size_of_val};

use image::Rgba;

pub struct QOIHeader {
    pub width: u32,   // image width in pixels (BE)
    pub height: u32,  // image height in pixels (BE)
    pub channels: u8, // 3 = RGB, 4 = RGBA
    pub colorspace: u8, // 0 = sRGB with linear alpha
                      // 1 = all channels linear
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct QOIRGBA {
    rgba: [u8; 4],
    // v: usize,
}

impl QOIRGBA {
    fn new() -> Self {
        Self {
            rgba: [0, 0, 0, 0],
            // v: 0,
        }
    }
}

const QOI_OP_INDEX: u8 = 0x00; /* 00xxxxxx */
const QOI_OP_DIFF: u8 = 0x40; /* 01xxxxxx */
const QOI_OP_LUMA: u8 = 0x80; /* 10xxxxxx */
const QOI_OP_RUN: u8 = 0xc0; /* 11xxxxxx */
const QOI_OP_RGB: u8 = 0xfe; /* 11111110 */
const QOI_OP_RGBA: u8 = 0xff; /* 11111111 */

const fn qoi_color_hash(c: &QOIRGBA) -> usize {
    c.rgba[0] as usize * 3
        + c.rgba[1] as usize * 5
        + c.rgba[2] as usize * 7
        + c.rgba[3] as usize * 11
}
const QOI_MAGIC: u32 =
    (('q' as u32) << 24) | (('o' as u32) << 16) | (('i' as u32) << 8) | ('f' as u32);
const QOI_HEADER_SIZE: usize = 14;

const QOI_PIXELS_MAX: u32 = 400000000;
const QOI_PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

pub fn qoi_write(filename: &str, data: Vec<[u8; 4]>, header: QOIHeader) -> usize {
    let mut file = File::create(filename).unwrap();

    let mut size = 0;
    let encoded = qoi_encode(data, header, &mut size).unwrap();

    file.write_all(&encoded).unwrap();
    file.flush().unwrap();

    size
}

fn qoi_encode(data: Vec<[u8; 4]>, header: QOIHeader, out_len: &mut usize) -> Option<Vec<u8>> {
    if data.is_empty()
        || header.width == 0
        || header.height == 0
        || header.channels < 3
        || header.channels > 4
        || header.colorspace > 1
        || header.height >= QOI_PIXELS_MAX / header.width
    {
        return None;
    }

    let max_size = header.width as usize * header.height as usize * (header.channels + 1) as usize
        + QOI_HEADER_SIZE
        + size_of_val(&QOI_PADDING);

    let mut bytes = Vec::with_capacity(max_size);

    // Write header information to file
    qoi_write_32(&mut bytes, QOI_MAGIC);
    qoi_write_32(&mut bytes, header.width);
    qoi_write_32(&mut bytes, header.height);
    bytes.push(header.channels);
    bytes.push(header.colorspace);

    let mut index = [QOIRGBA::new(); 64];

    let mut run = 0;
    let mut px_prev = QOIRGBA {
        rgba: Rgba::<u8>::from([0, 0, 0, 255]).0,
    };
    let mut px = px_prev.clone();

    let px_len = header.width as usize * header.height as usize;
    let px_end = px_len;
    // let channels = header.channels;

    for px_pos in 0..px_end {
        px.rgba = data[px_pos];

        if px == px_prev {
            run += 1;
            if run == 62 || px_pos == px_end {
                bytes.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }
        } else {
            if run > 0 {
                bytes.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }

            let index_pos = qoi_color_hash(&px) % 64;

            if index[index_pos] == px {
                bytes.push(QOI_OP_INDEX | index_pos as u8);
            } else {
                index[index_pos] = px;

                if px.rgba[3] == px_prev.rgba[3] {
                    let vr = px.rgba[0] as i16 - px_prev.rgba[0] as i16;
                    let vg = px.rgba[1] as i16 - px_prev.rgba[1] as i16;
                    let vb = px.rgba[2] as i16 - px_prev.rgba[2] as i16;

                    let vg_r = vr - vg;
                    let vg_b = vb - vg;

                    if vr > -3 && vr < 2 && vg > -3 && vg < 2 && vb > -3 && vb < 2 {
                        bytes.push(
                            QOI_OP_DIFF
                                | ((vr + 2) as u8) << 4
                                | ((vg + 2) as u8) << 2
                                | (vb + 2) as u8,
                        );
                    } else if vg_r > -9 && vg_r < 8 && vg > -33 && vg < 32 && vg_b > -9 && vg_b < 8
                    {
                        bytes.push(QOI_OP_LUMA | (vg + 32) as u8);
                        bytes.push(((vg_r + 8) as u8) << 4 | (vg_b + 8) as u8);
                    } else {
                        bytes.push(QOI_OP_RGB);
                        bytes.push(px.rgba[0]);
                        bytes.push(px.rgba[1]);
                        bytes.push(px.rgba[2]);
                    }
                } else {
                    bytes.push(QOI_OP_RGBA);
                    bytes.push(px.rgba[0]);
                    bytes.push(px.rgba[1]);
                    bytes.push(px.rgba[2]);
                    bytes.push(px.rgba[3]);
                }
            }
        }
        px_prev = px;
    }

    for i in 0..size_of_val(&QOI_PADDING) {
        bytes.push(QOI_PADDING[i]);
    }

    // bytes.

    *out_len = bytes.len();
    Some(bytes)
}

fn qoi_write_32(bytes: &mut Vec<u8>, v: u32) {
    bytes.push(((0xff000000 & v) >> 24) as u8);
    bytes.push(((0x00ff0000 & v) >> 16) as u8);
    bytes.push(((0x0000ff00 & v) >> 8) as u8);
    bytes.push((0x000000ff & v) as u8);
}
