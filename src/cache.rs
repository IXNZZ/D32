use std::collections::HashMap;
use std::{sync, thread};
use std::ops::{Deref, Index};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use ggez::{Context, GameError, GameResult, input};
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat};
use moka::sync::Cache;
use tracing::error;
use crate::asset::ImageData;
use itertools::Itertools;
use tracing_subscriber::filter::FilterExt;
use crate::asset;

#[derive(Clone, Debug)]
pub struct ImageMeta {
    pub src_x: f32,
    pub src_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: u32,
    pub height: u32,
    pub key: CacheKey
}

#[derive(Clone, Copy)]
pub struct ImageMark {
    pub max_width: u32,
    pub max_height: u32,
    pub current_width: u32,
    pub current_height: u32,
    pub next_height: u32,
}

impl ImageMark {
    pub fn new(max_width: u32, max_height: u32) -> Self {
        Self {
            max_width,
            max_height,
            current_width: 0,
            current_height: 0,
            next_height: 0,
        }
    }

    pub fn update(&mut self, key: CacheKey, data: &ImageData) -> ImageMeta {


        if data.width + self.current_width > self.max_width {
            self.current_width = 0;
            self.current_height = self.next_height;
        }

        let meta = ImageMeta {
            src_x: self.current_width as f32,
            src_y: self.current_height as f32,
            offset_x: data.offset_x,
            offset_y: data.offset_y,
            width: data.width,
            height: data.height,
            key
        };

        // update
        self.current_width += data.width;
        if self.next_height < self.current_height + data.height {
            self.next_height = self.current_height + data.height;
        }
        meta
    }
}

pub struct ImageValue {
    image: Image,
    meta: HashMap<u32, ImageMeta>
}

impl ImageValue {

    pub fn image(&self) -> Image {
        self.image.clone()
    }

    pub fn meta(&self, key: CacheMetaKey) -> Option<&ImageMeta> {
        self.meta.get(&key)
    }
}

type CacheDataKey = u32;
type CacheMetaKey = u32;

pub struct ImageCache {
    names: Cache<u32, String>,
    key_mark: Cache<CacheDataKey, ImageMark>,
    key_image: Cache<CacheDataKey, Arc<ImageValue>>,
    // temp_image: Cache<CacheDataKey, Arc<Vec<(ImageMeta, ImageData)>>>,
    load_sender: Sender<Vec<CacheKey>>,
    load_receiver: Receiver<(CacheDataKey, Vec<(ImageMeta, ImageData)>)>
}

impl ImageCache {

    pub fn new(data_dir: PathBuf) -> Self {
        let key_image = Cache::builder().time_to_idle(Duration::from_secs(5 * 60)).build();
        let key_mark = Cache::builder().time_to_idle(Duration::from_secs(5 * 60)).build();
        // let temp_image = Cache::builder().time_to_live(Duration::from_secs(1 * 60)).build();
        let (load_sender, receiver) = sync::mpsc::channel::<Vec<CacheKey>>();
        let (sender, load_receiver) = sync::mpsc::channel::<(CacheDataKey, Vec<(ImageMeta, ImageData)>)>();
        let mut names = Cache::new(255);
        names.insert(1, String::from("tiles"));
        names.insert(2, String::from("smTiles"));
        names.insert(3, String::from("objects"));
        let mut k = key_image.clone();
        // let mut t = temp_image.clone();
        let mut index: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut m = key_mark.clone();
        let n = names.clone();
        let cache = Self {
            names,
            key_mark,
            key_image,
            load_sender,
            load_receiver,
        };


        thread::spawn(move || {
            loop {
                if let Ok(keys) = receiver.recv() {
                    draw_image(&mut index, &mut m, &mut k, sender.clone(), keys, &n, data_dir.clone())
                }
            }
        });

        cache
    }

    pub fn add_name(&mut self, key: u32, name: String) {
        self.names.insert(key, name);
    }

    pub fn load_keys(&mut self, keys: &[CacheKey]) {
        self.load_sender.send(keys.to_vec()).unwrap();
    }
    pub fn load_key(&mut self, key: CacheKey) {
        self.load_sender.send(vec![key]).unwrap();
    }

    pub fn insert_key(&mut self, ctx: &mut Context) {
        self.load_receiver.try_iter().for_each(|(data_key, data)| {
            let mark = self.key_mark.get(&data_key).unwrap();
            let max_height = (mark.next_height / 2000 + 1 ) * 2000;
            let image = Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, 2000, max_height, 1);
            let mut canvas = Canvas::from_image(ctx, image.clone(), Color::from_rgba(0, 0, 0, 0));
            let mut meta_image = HashMap::new();
            if let Some(value) = self.key_image.get(&data_key) {
                canvas.draw(&value.image, DrawParam::default());
                value.meta.iter().for_each(|(k, v) |{
                    meta_image.insert(*k, v.clone());
                })
            }

            data.iter().for_each(|(meta, d)| {
                if d.bytes.len() > 0 && d.width > 0 && d.height > 0 {
                    let image1 = Image::from_pixels(ctx, d.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, d.width, d.height);
                    canvas.draw(&image1, DrawParam::new().dest(vec2(meta.src_x, meta.src_y)));
                }
                meta_image.insert(meta.key.get_meta_key(), meta.clone());
            });

            canvas.finish(ctx).unwrap();
            self.key_image.insert(data_key, Arc::new(ImageValue { image, meta: meta_image}))
        });
    }

    pub fn get(&mut self, key: &CacheDataKey) -> Option<Arc<ImageValue>> {
        // self.insert_key(ctx);
        self.key_image.get(key)
    }
}

fn draw_image<T: AsRef<Path>>(index: &mut HashMap<u32, Vec<u32>>,
              key_mark: &mut Cache<CacheDataKey, ImageMark>,
              cache: &mut Cache<CacheDataKey, Arc<ImageValue>>,
              sender: Sender<(CacheDataKey, Vec<(ImageMeta, ImageData)>)>,
              keys: Vec<CacheKey>,
    names: &Cache<u32, String>,
    data_dir: T) {
    let mut key_vec: Vec<CacheKey> = Vec::new();
    for key in keys {
        for count in 0..key.get_data_count() {
            key_vec.push(key.as_inc_index(count));
        }
    }

    let is_exists = |key: &CacheKey| {
       cache.contains_key(&key.get_data_key()) && cache.get(&key.get_data_key()).unwrap().meta.contains_key(&key.get_meta_key())
    };

    key_vec.into_iter()
        .unique()
        .group_by(|k|k.get_data_key())
        .into_iter()
        .for_each(|(data_key, group)| {
            let mut mark = if key_mark.contains_key(&data_key) {
                key_mark.get(&data_key).unwrap()
            } else {
                ImageMark::new(2000, 2000)
            };
            let md = group.filter(|key| !is_exists(key)).map(|key| {
                let data = load_image0(index, key, names, &data_dir);
                let meta = mark.update(key, &data);
                (meta, data)
            }).collect::<Vec<(ImageMeta, ImageData)>>();
            // temp.insert(data_key, Arc::new(md));
            sender.send((data_key, md)).unwrap();
            key_mark.insert(data_key, mark);
    });

}

fn load_image0<T: AsRef<Path>>(index: &mut HashMap<u32, Vec<u32>>, key: CacheKey, names: &Cache<u32, String>, data_dir: T) -> ImageData {
    let data_type = key.get_data_type();
    let file_idx = key.get_file_index();
    //如果没有找到名称映射表
    if !names.contains_key(&key.get_file_id()) {
        error!("没有找到名称映射表: key: {}", key.get_file_id());
        return ImageData::default();
    }
    let path = names.get(&key.get_file_id()).unwrap();

    if !index.contains_key(&key.get_idx_key()) {
        let path = get_file_name(&data_dir, path.as_str(), key.get_file_number(), data_type);
        if path.is_none() {
            error!("按类型映射文件类型出错(0,1,2): {}", data_type);
            return ImageData::default();
        }
        let path = path.unwrap();
        if key.get_idx_key() == 1 {
            index.insert(key.get_idx_key(), asset::read_index(path));
        } else {
            index.insert(key.get_idx_key(), asset::read_wzx(path));
        }
    }
    let path = get_file_name(&data_dir, path.as_str(), key.get_file_number(), 0);
    if path.is_none() {
        error!("按类型映射文件类型出错(0,1,2): {}", data_type);
        return ImageData::default();
    }
    let path = path.unwrap();

    let vec = index.get(&key.get_idx_key()).unwrap();
    let start = vec.get(file_idx);
    let end = vec.get(file_idx + 1);

    if start.is_some() {
        let end = if end.is_some() && data_type == 1 {
            *end.unwrap()
        } else { 16 };
        return if let Some(img) = asset::read_image(path, *start.unwrap(), end + *start.unwrap()) {
            img
        } else {
            ImageData::default()
        };
    }

    ImageData::default()
}

fn get_file_name<T: AsRef<Path>>(dir: &T, file_name: &str, file_number: u32, data_type: u32) -> Option<PathBuf> {
    let name = if file_number > 1 {
        format!("{}{}", file_name, file_number)
    } else {
        file_name.to_lowercase()
    };

    match data_type {
        1 => {
            Some(dir.as_ref().join(name).with_extension("idx"))
        },
        2 => {
            Some(dir.as_ref().join(name).with_extension("wzx"))
        },
        0 => {
            Some(dir.as_ref().join(name).with_extension("wzl"))
        }
        _ => {
            None
        }
    }
}


const DATA_ID_SHR: u32 = 55;
const DATA_ID_BITS: u32 = 0xFF;
const DATA_NUMBER_SHR: u32 = 45;
const DATA_NUMBER_BITS: u32 = 0x3FF;
const DATA_TYPE_SHR: u32 = 42;
const DATA_TYPE_BITS: u32 = 0x7;
const DATA_COUNT_SHR: u32 = 32;
const DATA_COUNT_BITS: u32 = 0x3FF;
const FILE_ID_SHR: u32 = 24;
const FILE_ID_BITS: u32 = 0xFF;
const FILE_NUMBER_SHR: u32 = 17;
const FILE_NUMBER_BITS: u32 = 0x7F;
const FILE_INDEX_SHR: u32 = 0;
const FILE_INDEX_BITS: u32 = 0x1FFFF;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CacheKey {
    long_key: u64,
}

impl CacheKey {
    pub fn new(long_key: u64) -> Self {
        Self { long_key }
    }

    pub fn from(data_id: u32, data_number: u32, data_type: u32, data_count: u32, file_id: u32, file_number: u32, file_index: u32) -> Self {
        let data_key = (data_id & DATA_ID_BITS) << 23 | (data_number & DATA_NUMBER_BITS) << 13 | (data_type & DATA_TYPE_BITS) << 10 | (data_count & DATA_COUNT_BITS);
        let meta_key = (file_id & FILE_ID_BITS) << FILE_ID_SHR | (file_number & FILE_NUMBER_BITS) << FILE_NUMBER_SHR | file_index & FILE_INDEX_BITS;
        Self { long_key: (data_key as u64) << 32 | meta_key as u64 }
    }

    pub fn from_cache(data_key: u32, meta_key: u32, data_count: u32) -> Self {
        let long_key = (data_key as u64) << 42 | (data_count as u64) << 32 | meta_key as u64;
        Self { long_key }
    }

    pub fn build_data_key(data_id: u32, data_number: u32, data_type: u32) -> CacheDataKey {
        (data_id & DATA_ID_BITS) << 13 | (data_number & DATA_NUMBER_BITS) << 3 | (data_type & DATA_TYPE_BITS)
    }

    pub fn as_inc_index(&self, inc: u32) -> Self {
        let long_key = CacheKey::from_cache(self.get_data_key(), self.get_meta_key(), 1).long_key;
        Self {long_key: long_key + inc as u64}
    }

    pub fn get_long_key(&self) -> u64 {
        self.long_key
    }

    pub fn get_idx_key(&self) -> u32 {
        self.get_data_type() << 15 | self.get_file_id() << 7 | self.get_file_number()
    }

    pub fn get_data_key(&self) -> CacheDataKey {
        (self.long_key >> 42) as u32
    }

    pub fn get_meta_key(&self) -> CacheMetaKey {
        (self.long_key as u32) & 0xFFFFFFFF
    }

    pub fn get_data_id(&self) -> u32 {
        (self.long_key >> DATA_ID_SHR) as u32 & DATA_ID_BITS
    }

    pub fn get_data_number(&self) -> u32 {
        (self.long_key >> DATA_NUMBER_SHR) as u32 & DATA_NUMBER_BITS
    }

    pub fn get_data_count(&self) -> u32 {
        (self.long_key >> DATA_COUNT_SHR) as u32 & DATA_COUNT_BITS
    }

    pub fn get_data_type(&self) -> u32 {
        (self.long_key >> DATA_TYPE_SHR) as u32 & DATA_TYPE_BITS
    }

    pub fn get_file_id(&self) -> u32 {
        (self.long_key >> FILE_ID_SHR) as u32 & FILE_ID_BITS
    }

    pub fn get_file_number(&self) -> u32 {
        (self.long_key >> FILE_NUMBER_SHR) as u32 & FILE_NUMBER_BITS
    }

    pub fn get_file_index(&self) -> usize {
        ((self.long_key >> FILE_INDEX_SHR) as u32 & FILE_INDEX_BITS) as usize
    }
}
