use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use bytes::{Buf, Bytes};
use flate2::{FlushDecompress, Status};
use tracing::{error, warn};



pub struct MapData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub back: u16,
    pub middle: u16,
    pub objects: u16,
    pub door_idx: u8,
    pub door_offset: u8,
    pub frame: u8,
    pub tick: u8,
    pub light: u8,
    pub objects_idx: u8,
    pub back_idx: u8,
    pub middle_idx: u8,
}

impl Tile {
    pub fn from(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let mut bytes = bytes;
        let back = bytes.get_u16_le();
        let middle = bytes.get_u16_le();
        let objects = bytes.get_u16_le();
        let door_idx = bytes.get_u8();
        let door_offset = bytes.get_u8();
        let frame = bytes.get_u8();
        let tick = bytes.get_u8();
        let objects_idx = bytes.get_u8();
        let light = bytes.get_u8();
        let back_idx = if len > 12 { bytes.get_u8() } else { 0 };
        let middle_idx = if len > 13 { bytes.get_u8() } else { 0 };

        Tile { back, middle, objects, door_idx, door_offset, frame, tick, objects_idx, light, back_idx, middle_idx }
    }
}

#[derive(Default)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub bytes: Bytes,
}

impl ImageData {
    pub fn from(src: &[u8]) -> Self {
        Self::from_head_data(&src[..16], &src[16..])
    }

    pub fn from_head_data(head: &[u8], data: &[u8]) -> Self {
        let mut body = head;
        let pixel = body.get_u8();
        let _compress = body.get_u8();
        let _reserve = body.get_u8();
        let _compress_level = body.get_u8();

        let width = body.get_u16_le() as u32;
        let height = body.get_u16_le() as u32;
        let offset_x = body.get_i16_le() as f32;
        let offset_y = body.get_i16_le() as f32;
        let length = body.get_u32_le();

        if length == 0 {
            let bytes = if data.len() > 0 {
                Bytes::from(byte_to_rgba(pixel, width as usize, height as usize, data))
            } else { Bytes::new() };
            Self { width, height, offset_x, offset_y, bytes }
        } else {
            // debug!("S3 length: {}, w: {}, h: {}, pixel: {}", length, width, height, pixel);
            let x = deflate_image(data, width * height);
            let data = byte_to_rgba(pixel, width as usize, height as usize, &x[..]);
            Self { width, height, offset_x, offset_y, bytes: Bytes::from(data) }
        }
    }
}

pub fn read_map_file(path: &str) -> Option<MapData> {
    let file =File::open(path);
    if file.is_err() {
        error!("未找到地图文件: {}", path);
        return None;
    }
    let mut file = file.unwrap();
    let file_size = file.metadata().unwrap().len();
    let mut header = [0u8; 52];
    file.read(&mut header).unwrap();
    let mut header = &header[..];
    let width = header.get_u16_le() as u32;
    let height = header.get_u16_le() as u32;
    let length = ((file_size as u32 - 52) / (width * height)) as usize;
    let mut body = Vec::with_capacity(file_size as usize -52);
    file.read_to_end(&mut body).unwrap();
    // let mut reader = BufReader::new(file);
    let mut tiles = Vec::with_capacity((width * height) as usize);
    for i in 0..width * height {
        let start = i as usize * length;
        let end = start + length;
        let tile = Tile::from(&body[start..end]);
        tiles.push(tile);
    }
    Some(MapData {width, height, tiles})
}

pub fn read_image(path: &str, start: u32, end: u32) -> Option<ImageData> {
    // debug!("S1 start: {}, end: {}, len: , path: {}", start, end, path);
    let file =File::open(path);
    if file.is_err() {
        error!("未找到资源文件: {}", path);
        return None;
    }
    let file = file.unwrap();
    let mut reader = BufReader::new(file);
    let mut data = vec![0;(end - start) as usize];
    reader.seek(SeekFrom::Start(start as u64)).unwrap();
    reader.read(&mut data[..]).unwrap();
    if end - start == 16 {
        let mut x = &data[12..];
        let x = x.get_u32_le();
        if x > 0 {
            let mut i = vec![0; x as usize];
            reader.seek(SeekFrom::Start(start as u64 + 16)).unwrap();
            reader.read(&mut i[..]).unwrap();
            return Some(ImageData::from_head_data(&data[..], &i[..]));
        }
    }
    Some(ImageData::from(&data[..]))
}

pub fn read_index(path: &str) -> Vec<u32> {
    let file =File::open(path);
    if file.is_err() {
        error!("未找到索引文件: {}", path);
        return Vec::new();
    }
    let mut file = file.unwrap();
    let len = file.metadata().unwrap().len();
    let mut data = Vec::with_capacity(len as usize);
    file.read_to_end(&mut data).unwrap();
    let mut data = &data[..];
    let len = data.len() / 4;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(data.get_u32_le());
    }
    result
}

pub fn read_wzx(path: &str) -> Vec<u32> {
    // println!("read_wzx {}", path);
    let file = File::open(path);
    if !file.is_ok() {
        return vec![48];
    }
    let mut file = file.unwrap();
    let len = file.metadata().unwrap().len();
    let mut data = Vec::with_capacity(len as usize);
    file.read_to_end(&mut data).unwrap();
    let mut data = &data[48..];
    let len = data.len() / 4;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(data.get_u32_le());
    }
    result
}

fn deflate_image(input: &[u8], size: u32) -> Vec<u8> {
    let capacity = size * 4;
    let mut rs: Vec<u8> = Vec::with_capacity(capacity as usize);
    let status = flate2::Decompress::new(true).decompress_vec(input, &mut rs, FlushDecompress::Finish).unwrap();
    if status != Status::StreamEnd {
        warn!("input: {}, output: {}, status: {:?}, size: {}, size*2: {}", input.len(), rs.len(), status, size, size *2);
        // return deflate_image(input, size * 2);
    }
    rs
}

fn byte_to_rgba(pixel: u8, width: usize, height: usize, bytes: &[u8]) -> Vec<u8> {

    if pixel == 3 {
        let mut result = Vec::with_capacity(bytes.len() * 4);
        let new_width = bytes.len() / height;
        let n_width = (width + 3) / 4 * 4;
        let n_width = if n_width > new_width { new_width } else { n_width };
        let n_width = if new_width > n_width { width } else { n_width };
        for i in 0..height {
            for j in 0..width {
                let x = bytes[(height - i - 1) * n_width + j] as usize * 4;
                result.extend_from_slice(&PALETTE_RGBA[x..x+4]);
            }
        }
        return result;
    } else {
        let mut result = Vec::with_capacity(bytes.len() * 2);
        let new_width = bytes.len() / height / 2;
        let n_width = (width + 3) / 4 * 4;
        let n_width = if n_width > new_width { new_width } else { n_width };
        let n_width = if new_width > n_width { width } else { n_width };
        for i in 0..height {
            for j in 0..width {
                let p = (height - i - 1) * n_width * 2 + j * 2;
                // let r = bytes[p + 1] & 0xF8;
                // let g = (((bytes[p + 1] & 0x7) << 3) | (bytes[p] >> 5)) * 4;
                // let b = (bytes[p] & 0x1F) * 8;
                result.push(bytes[p + 1] & 0xF8); //R
                result.push( (((bytes[p + 1] & 0x7) << 3) | (bytes[p] >> 5)) * 4); //G
                result.push((bytes[p] & 0x1F) * 8); //G
                result.push(if bytes[p] == 0 && bytes[p + 1] == 0 { 0 } else { 255 }); //A
            }
        }
        return result;
    }
}

const PALETTE_RGBA: [u8; 1024] = [
    0x00,0x00,0x00,0x00,0x80,0x00,0x00,0xFF,0x00,0x80,0x00,0xFF,0x80,0x80,0x00,0xFF,0x00,0x00,0x80,0xFF,0x80,0x00,0x80,0xFF,0x00,0x80,0x80,0xFF,0xC0,0xC0,0xC0,0xFF,
    0x55,0x80,0x97,0xFF,0x9D,0xB9,0xC8,0xFF,0x7B,0x73,0x73,0xFF,0x2D,0x29,0x29,0xFF,0x5A,0x52,0x52,0xFF,0x63,0x5A,0x5A,0xFF,0x42,0x39,0x39,0xFF,0x1D,0x18,0x18,0xFF,
    0x18,0x10,0x10,0xFF,0x29,0x18,0x18,0xFF,0x10,0x08,0x08,0xFF,0xF2,0x79,0x71,0xFF,0xE1,0x67,0x5F,0xFF,0xFF,0x5A,0x5A,0xFF,0xFF,0x31,0x31,0xFF,0xD6,0x5A,0x52,0xFF,
    0x94,0x10,0x00,0xFF,0x94,0x29,0x18,0xFF,0x39,0x08,0x00,0xFF,0x73,0x10,0x00,0xFF,0xB5,0x18,0x00,0xFF,0xBD,0x63,0x52,0xFF,0x42,0x18,0x10,0xFF,0xFF,0xAA,0x99,0xFF,
    0x5A,0x10,0x00,0xFF,0x73,0x39,0x29,0xFF,0xA5,0x4A,0x31,0xFF,0x94,0x7B,0x73,0xFF,0xBD,0x52,0x31,0xFF,0x52,0x21,0x10,0xFF,0x7B,0x31,0x18,0xFF,0x2D,0x18,0x10,0xFF,
    0x8C,0x4A,0x31,0xFF,0x94,0x29,0x00,0xFF,0xBD,0x31,0x00,0xFF,0xC6,0x73,0x52,0xFF,0x6B,0x31,0x18,0xFF,0xC6,0x6B,0x42,0xFF,0xCE,0x4A,0x00,0xFF,0xA5,0x63,0x39,0xFF,
    0x5A,0x31,0x18,0xFF,0x2A,0x10,0x00,0xFF,0x15,0x08,0x00,0xFF,0x3A,0x18,0x00,0xFF,0x08,0x00,0x00,0xFF,0x29,0x00,0x00,0xFF,0x4A,0x00,0x00,0xFF,0x9D,0x00,0x00,0xFF,
    0xDC,0x00,0x00,0xFF,0xDE,0x00,0x00,0xFF,0xFB,0x00,0x00,0xFF,0x9C,0x73,0x52,0xFF,0x94,0x6B,0x4A,0xFF,0x73,0x4A,0x29,0xFF,0x52,0x31,0x18,0xFF,0x8C,0x4A,0x18,0xFF,
    0x88,0x44,0x11,0xFF,0x4A,0x21,0x00,0xFF,0x21,0x18,0x10,0xFF,0xD6,0x94,0x5A,0xFF,0xC6,0x6B,0x21,0xFF,0xEF,0x6B,0x00,0xFF,0xFF,0x77,0x00,0xFF,0xA5,0x94,0x84,0xFF,
    0x42,0x31,0x21,0xFF,0x18,0x10,0x08,0xFF,0x29,0x18,0x08,0xFF,0x21,0x10,0x00,0xFF,0x39,0x29,0x18,0xFF,0x8C,0x63,0x39,0xFF,0x42,0x29,0x10,0xFF,0x6B,0x42,0x18,0xFF,
    0x7B,0x4A,0x18,0xFF,0x94,0x4A,0x00,0xFF,0x8C,0x84,0x7B,0xFF,0x6B,0x63,0x5A,0xFF,0x4A,0x42,0x39,0xFF,0x29,0x21,0x18,0xFF,0x46,0x39,0x29,0xFF,0xB5,0xA5,0x94,0xFF,
    0x7B,0x6B,0x5A,0xFF,0xCE,0xB1,0x94,0xFF,0xA5,0x8C,0x73,0xFF,0x8C,0x73,0x5A,0xFF,0xB5,0x94,0x73,0xFF,0xD6,0xA5,0x73,0xFF,0xEF,0xA5,0x4A,0xFF,0xEF,0xC6,0x8C,0xFF,
    0x7B,0x63,0x42,0xFF,0x6B,0x56,0x39,0xFF,0xBD,0x94,0x5A,0xFF,0x63,0x39,0x00,0xFF,0xD6,0xC6,0xAD,0xFF,0x52,0x42,0x29,0xFF,0x94,0x63,0x18,0xFF,0xEF,0xD6,0xAD,0xFF,
    0xA5,0x8C,0x63,0xFF,0x63,0x5A,0x4A,0xFF,0xBD,0xA5,0x7B,0xFF,0x5A,0x42,0x18,0xFF,0xBD,0x8C,0x31,0xFF,0x35,0x31,0x29,0xFF,0x94,0x84,0x63,0xFF,0x7B,0x6B,0x4A,0xFF,
    0xA5,0x8C,0x5A,0xFF,0x5A,0x4A,0x29,0xFF,0x9C,0x7B,0x39,0xFF,0x42,0x31,0x10,0xFF,0xEF,0xAD,0x21,0xFF,0x18,0x10,0x00,0xFF,0x29,0x21,0x00,0xFF,0x9C,0x6B,0x00,0xFF,
    0x94,0x84,0x5A,0xFF,0x52,0x42,0x18,0xFF,0x6B,0x5A,0x29,0xFF,0x7B,0x63,0x21,0xFF,0x9C,0x7B,0x21,0xFF,0xDE,0xA5,0x00,0xFF,0x5A,0x52,0x39,0xFF,0x31,0x29,0x10,0xFF,
    0xCE,0xBD,0x7B,0xFF,0x63,0x5A,0x39,0xFF,0x94,0x84,0x4A,0xFF,0xC6,0xA5,0x29,0xFF,0x10,0x9C,0x18,0xFF,0x42,0x8C,0x4A,0xFF,0x31,0x8C,0x42,0xFF,0x10,0x94,0x29,0xFF,
    0x08,0x18,0x10,0xFF,0x08,0x18,0x18,0xFF,0x08,0x29,0x10,0xFF,0x18,0x42,0x29,0xFF,0xA5,0xB5,0xAD,0xFF,0x6B,0x73,0x73,0xFF,0x18,0x29,0x29,0xFF,0x18,0x42,0x4A,0xFF,
    0x31,0x42,0x4A,0xFF,0x63,0xC6,0xDE,0xFF,0x44,0xDD,0xFF,0xFF,0x8C,0xD6,0xEF,0xFF,0x73,0x6B,0x39,0xFF,0xF7,0xDE,0x39,0xFF,0xF7,0xEF,0x8C,0xFF,0xF7,0xE7,0x00,0xFF,
    0x6B,0x6B,0x5A,0xFF,0x5A,0x8C,0xA5,0xFF,0x39,0xB5,0xEF,0xFF,0x4A,0x9C,0xCE,0xFF,0x31,0x84,0xB5,0xFF,0x31,0x52,0x6B,0xFF,0xDE,0xDE,0xD6,0xFF,0xBD,0xBD,0xB5,0xFF,
    0x8C,0x8C,0x84,0xFF,0xF7,0xF7,0xDE,0xFF,0x00,0x08,0x18,0xFF,0x08,0x18,0x39,0xFF,0x08,0x10,0x29,0xFF,0x08,0x18,0x00,0xFF,0x08,0x29,0x00,0xFF,0x00,0x52,0xA5,0xFF,
    0x00,0x7B,0xDE,0xFF,0x10,0x29,0x4A,0xFF,0x10,0x39,0x6B,0xFF,0x10,0x52,0x8C,0xFF,0x21,0x5A,0xA5,0xFF,0x10,0x31,0x5A,0xFF,0x10,0x42,0x84,0xFF,0x31,0x52,0x84,0xFF,
    0x18,0x21,0x31,0xFF,0x4A,0x5A,0x7B,0xFF,0x52,0x6B,0xA5,0xFF,0x29,0x39,0x63,0xFF,0x10,0x4A,0xDE,0xFF,0x29,0x29,0x21,0xFF,0x4A,0x4A,0x39,0xFF,0x29,0x29,0x18,0xFF,
    0x4A,0x4A,0x29,0xFF,0x7B,0x7B,0x42,0xFF,0x9C,0x9C,0x4A,0xFF,0x5A,0x5A,0x29,0xFF,0x42,0x42,0x14,0xFF,0x39,0x39,0x00,0xFF,0x59,0x59,0x00,0xFF,0xCA,0x35,0x2C,0xFF,
    0x6B,0x73,0x21,0xFF,0x29,0x31,0x00,0xFF,0x31,0x39,0x10,0xFF,0x31,0x39,0x18,0xFF,0x42,0x4A,0x00,0xFF,0x52,0x63,0x18,0xFF,0x5A,0x73,0x29,0xFF,0x31,0x4A,0x18,0xFF,
    0x18,0x21,0x00,0xFF,0x18,0x31,0x00,0xFF,0x18,0x39,0x10,0xFF,0x63,0x84,0x4A,0xFF,0x6B,0xBD,0x4A,0xFF,0x63,0xB5,0x4A,0xFF,0x63,0xBD,0x4A,0xFF,0x5A,0x9C,0x4A,0xFF,
    0x4A,0x8C,0x39,0xFF,0x63,0xC6,0x4A,0xFF,0x63,0xD6,0x4A,0xFF,0x52,0x84,0x4A,0xFF,0x31,0x73,0x29,0xFF,0x63,0xC6,0x5A,0xFF,0x52,0xBD,0x4A,0xFF,0x10,0xFF,0x00,0xFF,
    0x18,0x29,0x18,0xFF,0x4A,0x88,0x4A,0xFF,0x4A,0xE7,0x4A,0xFF,0x00,0x5A,0x00,0xFF,0x00,0x88,0x00,0xFF,0x00,0x94,0x00,0xFF,0x00,0xDE,0x00,0xFF,0x00,0xEE,0x00,0xFF,
    0x00,0xFB,0x00,0xFF,0x4A,0x5A,0x94,0xFF,0x63,0x73,0xB5,0xFF,0x7B,0x8C,0xD6,0xFF,0x6B,0x7B,0xD6,0xFF,0x77,0x88,0xFF,0xFF,0xC6,0xC6,0xCE,0xFF,0x94,0x94,0x9C,0xFF,
    0x9C,0x94,0xC6,0xFF,0x31,0x31,0x39,0xFF,0x29,0x18,0x84,0xFF,0x18,0x00,0x84,0xFF,0x4A,0x42,0x52,0xFF,0x52,0x42,0x7B,0xFF,0x63,0x5A,0x73,0xFF,0xCE,0xB5,0xF7,0xFF,
    0x8C,0x7B,0x9C,0xFF,0x77,0x22,0xCC,0xFF,0xDD,0xAA,0xFF,0xFF,0xF0,0xB4,0x2A,0xFF,0xDF,0x00,0x9F,0xFF,0xE3,0x17,0xB3,0xFF,0xFF,0xFB,0xF0,0xFF,0xA0,0xA0,0xA4,0xFF,
    0x80,0x80,0x80,0xFF,0xFF,0x00,0x00,0xFF,0x00,0xFF,0x00,0xFF,0xFF,0xFF,0x00,0xFF,0x00,0x00,0xFF,0xFF,0xFF,0x00,0xFF,0xFF,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
];