// use std::collections::{HashMap, HashSet};
// use std::ops::Deref;
// use std::sync::Arc;
// use std::time::{Duration, Instant};
// use ggez::{Context, GameError, GameResult};
// use ggez::glam::vec2;
// use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat};
// use itertools::Itertools;
// use moka::sync::Cache;
// use tracing::error;
// use crate::asset;
// use crate::asset::ImageData;
//
// #[derive(Clone)]
// pub struct ImageMeta {
//     pub src_x: f32,
//     pub src_y: f32,
//     pub offset_x: f32,
//     pub offset_y: f32,
//     pub width: u32,
//     pub height: u32,
// }
// struct ImageValue {
//     max_width: f32,
//     max_height: f32,
//     current_width: f32,
//     current_height: f32,
//     next_height: f32,
//     // pub key: u32,
// }
//
// impl ImageValue {
//     pub fn new(width: f32, height: f32) -> Self {
//         Self {
//             max_width: width,
//             max_height: height,
//             current_width: 0.,
//             current_height: 0.,
//             next_height: 0.,
//         }
//     }
//
//     pub fn validate(&self, data: &ImageData) -> GameResult {
//         if data.height as f32 + self.current_height > self.max_height {
//             return Err(GameError::CustomError(String::from("超出图片最大尺寸")));
//         }
//
//         if data.width as f32 + self.current_width > self.max_width && data.height as f32 + self.next_height > self.max_height {
//             return Err(GameError::CustomError(String::from("超出图片最大尺寸")));
//         }
//
//         Ok(())
//     }
//
//     pub fn update(&mut self, data: &ImageData) -> ImageMeta {
//
//
//         if data.width as f32 + self.current_width > self.max_width {
//             self.current_width = 0.;
//             self.current_height = self.next_height;
//         }
//
//         let meta = ImageMeta {
//             src_x: self.current_width,
//             src_y: self.current_height,
//             offset_x: data.offset_x,
//             offset_y: data.offset_y,
//             width: data.width,
//             height: data.height,
//         };
//
//         // update
//         self.current_width += data.width as f32;
//         if self.next_height < self.current_height + data.height as f32 {
//             self.next_height = self.current_height + data.height as f32;
//         }
//         meta
//     }
// }
// pub type ImageCache = Arc<(Image, HashMap<u32, ImageMeta>)>;
// pub struct ImageCacheManager {
//     data_dir: String,
//     file_names: HashMap<u32, String>,
//     idx: Cache<u32, Arc<Vec<u32>>>,
//     // wzx: Cache<u32, Arc<Vec<u32>>>,
//     value: HashMap<u32, ImageValue>,
//     cache: Cache<u32, ImageCache>
// }
//
// impl ImageCacheManager {
//
//     pub fn new(data_dir: &str, max_capacity: u64) -> Self {
//         let mut convert = HashMap::new();
//         convert.insert(1, String::from("tiles"));
//         convert.insert(2, String::from("smTiles"));
//         convert.insert(3, String::from("objects"));
//         // idle cache build
//         Self {
//             data_dir: String::from(data_dir),
//             file_names: convert,
//             idx: Cache::new(100),
//             // wzx: Cache::new(100),
//             value: HashMap::new(),
//             cache: Cache::builder().time_to_idle(Duration::from_secs(10 * 60)).build(),
//         }
//     }
//
//
//     fn build_file_path(&self, cache_key: &CacheKey, data_type: u32) -> Option<String> {
//         let file = self.file_names.get(&cache_key.get_file_id());
//         if file.is_none() {
//             error!("不存在文件名映射-file:{}, seq: {}, type: {}", cache_key.get_file_id(), cache_key.get_file_number(), data_type);
//             return None;
//         }
//
//         let file = file.unwrap().as_str();
//         convert_file_map(self.data_dir.as_str(), file, cache_key.get_file_number(), data_type)
//     }
//
//     fn load_image_index(&mut self, cache_key: &CacheKey) -> Option<(u32, u32)> {
//         // let map_index_key = build_idx_key(idx_type, file_key, seq_key);
//         let map_index_key = cache_key.get_idx_key();
//         if !self.idx.contains_key(&map_index_key) {
//             let file_path = self.build_file_path(cache_key, cache_key.get_data_type());
//             if file_path.is_none() {
//                 error!("未实现的文件类型映射: name: {}, {}", cache_key.get_file_id(), cache_key.get_data_type());
//                 return None;
//             }
//
//             let file_path = file_path.unwrap();
//             let idx_vec = if cache_key.get_data_type() == 1 {
//                 asset::read_index(file_path.as_str())
//             } else {
//                 asset::read_wzx(file_path.as_str())
//             };
//             self.idx.insert(map_index_key, Arc::new(idx_vec));
//         }
//
//         if let Some(d) = self.idx.get(&map_index_key) {
//             let i0 = d.get(cache_key.get_file_index());
//             if i0.is_some() {
//                 let i1 = d.get(cache_key.get_file_index() + 1);
//                 if cache_key.get_data_type() == 1 && i1.is_some() {
//                     return Some((*i0.unwrap(), *i1.unwrap()))
//                 }
//                 return Some((*i0.unwrap(), *i0.unwrap() + 16))
//             }
//         }
//
//         None
//     }
//
//
//     fn load_image_data(&mut self, cache_key: &CacheKey) -> Option<ImageData> {
//         // let (file_key, seq_key, idx) = split_meta_key(meta_key);
//         // let (_, _, idx_type) = split_image_key(image_key);
//         let index = self.load_image_index(cache_key);
//         if index.is_none() {
//             error!("未加载图片索引: type: {}, file: {}, seq: {}, idx: {}",
//                 cache_key.get_data_type(), cache_key.get_file_id(), cache_key.get_file_number(), cache_key.get_file_index());
//             return None;
//         }
//         let (start, end) = index.unwrap();
//         let file_path = self.build_file_path(cache_key, 0);
//         if file_path.is_none() {
//             error!("未实现的文件类型映射: name: {}, {}", cache_key.get_file_id(), cache_key.get_data_type());
//             return None;
//         }
//         let file_path = file_path.unwrap();
//         // println!("")
//         asset::read_image(file_path.as_str(), start, end)
//     }
//
//     fn draw_image(&mut self, image_key: u32, image: Image, ctx: &mut Context, data: ImageData) -> (Image, ImageMeta) {
//         let image_value = self.value.get_mut(&image_key).unwrap();
//         let image = if image_value.validate(&data).is_err() {
//             let inc_height = ctx.gfx.size().1 + image_value.max_height;
//             let new_img = Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, image_value.max_width as u32, inc_height as u32, 1);
//             let mut canvas = Canvas::from_image(ctx, new_img.clone(), None);
//             canvas.draw(&image, DrawParam::default());
//             canvas.finish(ctx).unwrap();
//             new_img
//         } else {
//             image
//         };
//         let mut meta = ImageMeta {
//             src_x: image_value.current_width,
//             src_y: image_value.current_height,
//             offset_x: data.offset_x,
//             offset_y: data.offset_y,
//             width: data.width,
//             height: data.height,
//         };
//
//         image_value.update(&data);
//
//         if !data.bytes.is_empty() && data.width > 0 && data.height > 0 {
//             let mut canvas = Canvas::from_image(ctx, image.clone(), None);
//             let data_image = Image::from_pixels(ctx, data.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, data.width, data.height);
//             canvas.draw(&data_image, DrawParam::default().dest(vec2(image_value.current_width - data.width as f32, image_value.current_height)));
//             canvas.finish(ctx).unwrap();
//         }
//
//         (image, meta)
//     }
//
//     fn check_set_by_key(&mut self, cache_key: &CacheKey) -> HashSet<u64> {
//         let data_key = cache_key.get_data_key();
//         let data_count = cache_key.get_data_count();
//         let mut data_set: HashSet<u64> = HashSet::with_capacity(data_count as usize);
//         for i in 0..data_count {
//             let cache_key = cache_key.as_inc_index(i);
//             if let Some(x) = self.cache.get(&data_key) {
//                 if x.1.contains_key(&cache_key.get_meta_key()) {
//                     continue;
//                 }
//             }
//             data_set.insert(cache_key.long_key);
//         }
//         data_set
//     }
//
//     fn load_by_set(&mut self, ctx: &mut Context, key_set: HashSet<u64>) {
//         let time = Instant::now();
//         let group_key: Vec<(u32, Vec<CacheKey>)> = key_set.into_iter()
//             .group_by(|x| CacheKey::new(*x).get_data_key()).into_iter()
//             .map(|(x, v)| {
//                 (x, v.map(|x| CacheKey::new(x)).collect())
//             })
//             .collect();
//         println!("time group: {:?}", time.elapsed());
//         let (width, height) = ctx.gfx.size();
//         for (data_key, meta_keys) in group_key {
//             if !self.cache.contains_key(&data_key) {
//                 self.value.insert(data_key, ImageValue::new(width, height));
//             }
//
//             let mut meta_hash: HashMap<u32, ImageMeta> = HashMap::new();
//             let mut image_data_vec = Vec::with_capacity(meta_keys.len());
//
//             for meta_key in meta_keys {
//                 let data = if let Some(data) = self.load_image_data(&meta_key) {
//                     // println!("load image: {}|{}|{}|{}", data.width, data.height, data.offset_x, data.offset_y);
//                     data
//                 } else {
//                     ImageData::default()
//                 };
//                 let image_value = self.value.get_mut(&data_key).unwrap();
//                 if image_value.validate(&data).is_err() {
//                     image_value.max_height += height;
//                 }
//                 let meta_data = image_value.update(&data);
//
//                 image_data_vec.push((meta_key, meta_data, data))
//             }
//             println!("time load image: {:?}", time.elapsed());
//             let image_value = self.value.get_mut(&data_key).unwrap();
//             let image = Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, image_value.max_width as u32, image_value.max_height as u32, 1);
//             let mut canvas = Canvas::from_image(ctx, image.clone(), Color::from_rgba(0, 0, 0, 0));
//
//             for (key, meta, data) in image_data_vec {
//                 let data_image = Image::from_pixels(ctx, data.bytes.as_ref(), ImageFormat::Rgba8UnormSrgb, data.width, data.height);
//                 canvas.draw(&data_image, DrawParam::new().dest(vec2(meta.src_x, meta.src_y)));
//                 meta_hash.insert(key.get_meta_key(), meta);
//             }
//             println!("time draw1: {:?}", time.elapsed());
//             if let Some(v) = self.cache.get(&data_key) {
//                 canvas.draw(&v.0, DrawParam::default());
//                 for (k, v) in &v.1 {
//                     meta_hash.insert(*k, v.clone());
//                 }
//                 println!("time draw2: {:?}", time.elapsed());
//             }
//             canvas.finish(ctx).unwrap();
//             self.cache.insert(data_key, Arc::new((image, meta_hash)));
//             println!("time finish: {:?}", time.elapsed());
//         }
//     }
//
//     pub fn put_cache(&mut self, ctx: &mut Context, cache_key: &CacheKey) {
//         let set = self.check_set_by_key(cache_key);
//         if !set.is_empty() {
//             let instant = Instant::now();
//             self.load_by_set(ctx, set);
//             println!("instant: {:?}", instant.elapsed())
//         }
//
//     }
//
//     pub fn put_by_key(&mut self, ctx: &mut Context, cache_key: &CacheKey) {
//         // let (image_key, meta_key, image_count) = split_key(long_key);
//         // let cache_key = CacheKey::new(long_key);
//         let data_key = cache_key.get_data_key();
//
//         if !self.value.contains_key(&data_key) {
//             let (width, height) = ctx.gfx.size();
//             self.value.insert(data_key, ImageValue::new(width, height));
//         }
//
//         let cache = if self.cache.contains_key(&data_key) {
//             self.cache.get(&data_key).unwrap()
//         } else {
//             let (width, height) = ctx.gfx.size();
//             let image = Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, width as u32, height as u32, 1);
//             Arc::new((image, HashMap::new()))
//         };
//
//
//         let (image, image_map) = cache.deref();
//         let mut image_map = image_map.clone();
//         // let image = image.clone();
//         let mut image_vec = Vec::new();
//         let data_count = cache_key.get_data_count();
//         // let meta_key = cache_key.get_meta_key();
//         for i in 0..data_count {
//             // let meta_key= meta_key + i;
//             let cache_key = cache_key.as_inc_index(i);
//             // println!("cache: {:?}", cache_key);
//             if image_map.contains_key(&cache_key.get_meta_key()) {
//                 continue;
//             }
//
//             // load image data
//             let data = if let Some(data) = self.load_image_data(&cache_key) {
//                 println!("load image: {}|{}|{}|{}", data.width, data.height, data.offset_x, data.offset_y);
//                 data
//             } else {
//                 ImageData::default()
//             };
//             // let image = cache.0;
//
//
//             // let image = if image_value.validate(&data).is_err() {
//             //     let inc_height = ctx.gfx.size().1 + image_value.max_height;
//             //     Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, image_value.max_width as u32, inc_height as u32, 1)
//             // } else {
//             //     image
//             // };
//
//             let (image, meta) = self.draw_image(data_key, image.clone(), ctx, data);
//             // meta.src_x = image_value.current_width - meta.width as f32;
//             // meta.src_y = image_value.current_height;
//             image_map.insert(cache_key.get_meta_key(), meta);
//             image_vec.insert(0, image);
//         }
//         if image_vec.len() > 0 {
//             self.cache.insert(data_key, Arc::new((image_vec.get(0).unwrap().clone(), image_map)))
//         }
//
//     }
//
//     // pub fn put_by_keys(&mut self, ctx: &mut Context, long_keys: &[u64]) {
//     //     for x in long_keys {
//     //         self.put_by_key(ctx, *x);
//     //     }
//     // }
//
//     pub fn get(&self, data_key: u32) -> Option<ImageCache> {
//         // if self.cache.get(key) {  }
//         self.cache.get(&data_key)
//     }
// }
//
// /// 返回值说明: (data_key, meta_key, data_count)
// // pub fn split_key(key: u64) -> (u32, u32, u32) {
// //     ((key >> 32) as u32 & 0xFFFFFC00, key as u32, (key >> 32) as u32 & 0x3FF)
// // }
// //
// // /// 返回值说明: (file_key, file_number, file_index)
// // pub fn split_meta_key(meta_key: u32) -> (u32, u32, u32) {
// //     (meta_key >> 24, (meta_key >> 17) & 0x7F, meta_key & 0x1FFFF)
// // }
// //
// // /// 返回值说明: (data_key, data_number, data_type)
// // pub fn split_image_key(data_key: u32) -> (u32, u32, u32) {
// //     (data_key >> 23, (data_key >> 13) & 0x3FF, (data_key >> 10) & 0x7)
// // }
//
//
//
// fn convert_file_map(dir: &str, file_name: &str, file_number: u32, data_type: u32) -> Option<String> {
//     let name = if file_number > 1 {
//         format!("{}/{}{}", dir, file_name, file_number)
//     } else {
//         format!("{}/{}", dir, file_name)
//     };
//     match data_type {
//         1 => {
//             Some(format!("{}.idx", name))
//         },
//         2 => {
//             Some(format!("{}.wzx", name))
//         },
//         0 => {
//             Some(format!("{}.wzl", name))
//         }
//         _ => {
//             None
//         }
//     }
// }
//
// // fn build_idx_key(idx_type: u32, file_key: u32, seq_key: u32) -> u32 {
// //     idx_type << 24 | file_key << 8 | seq_key
// // }
// //
// // pub fn build_data_key(data_key: u32, data_number: u32, data_type: u32, data_count: u32) -> u32 {
// //     (data_key & 0xFF) << 23 | (data_number & 0x3FF) << 13 | (data_type & 0x7) << 10 | (data_count & 0x3FF)
// // }
// //
// // pub fn build_meta_key(file_key: u32, file_number: u32, file_index: u32) -> u32 {
// //     (file_key & 0xFF) << 24 | (file_number & 0x7F) << 17 | file_index & 0x1FFFF
// // }
// //
// //
// // pub fn build_cache_key(data_key: u32, data_number: u32, data_type: u32, data_count: u32, file_key: u32, file_number: u32, file_index: u32) -> u64 {
// //     let data_key = (data_key & 0xFF) << 23 | (data_number & 0x3FF) << 13 | (data_type & 0x7) << 10 | (data_count & 0x3FF);
// //     let meta_key = (file_key & 0xFF) << 24 | (file_number & 0x7F) << 17 | file_index & 0x1FFFF;
// //     (data_key as u64) << 32 | meta_key as u64
// // }
//
// const DATA_ID_SHR: u32 = 55;
// const DATA_ID_BITS: u32 = 0xFF;
// const DATA_NUMBER_SHR: u32 = 45;
// const DATA_NUMBER_BITS: u32 = 0x3FF;
// const DATA_TYPE_SHR: u32 = 42;
// const DATA_TYPE_BITS: u32 = 0x7;
// const DATA_COUNT_SHR: u32 = 32;
// const DATA_COUNT_BITS: u32 = 0x3FF;
// const FILE_ID_SHR: u32 = 24;
// const FILE_ID_BITS: u32 = 0xFF;
// const FILE_NUMBER_SHR: u32 = 17;
// const FILE_NUMBER_BITS: u32 = 0x7F;
// const FILE_INDEX_SHR: u32 = 0;
// const FILE_INDEX_BITS: u32 = 0x1FFFF;
//
// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub struct CacheKey {
//     long_key: u64,
// }
//
// impl CacheKey {
//     pub fn new(long_key: u64) -> Self {
//         Self { long_key }
//     }
//
//     pub fn from(data_id: u32, data_number: u32, data_type: u32, data_count: u32, file_id: u32, file_number: u32, file_index: u32) -> Self {
//         let data_key = (data_id & DATA_ID_BITS) << 23 | (data_number & DATA_NUMBER_BITS) << 13 | (data_type & DATA_TYPE_BITS) << 10 | (data_count & DATA_COUNT_BITS);
//         let meta_key = (file_id & FILE_ID_BITS) << FILE_ID_SHR | (file_number & FILE_NUMBER_BITS) << FILE_NUMBER_SHR | file_index & FILE_INDEX_BITS;
//         Self { long_key: (data_key as u64) << 32 | meta_key as u64 }
//     }
//
//     pub fn from_cache(data_key: u32, meta_key: u32, data_count: u32) -> Self {
//         let long_key = (data_key as u64) << 42 | (data_count as u64) << 32 | meta_key as u64;
//         Self { long_key }
//     }
//
//     pub fn as_inc_index(&self, inc: u32) -> Self {
//         Self {long_key: self.long_key + inc as u64}
//     }
//
//
//
//
//     pub fn get_long_key(&self) -> u64 {
//         self.long_key
//     }
//
//     pub fn get_idx_key(&self) -> u32 {
//         self.get_data_type() << 15 | self.get_file_id() << 7 | self.get_file_number()
//     }
//
//     pub fn get_data_key(&self) -> u32 {
//         (self.long_key >> 42) as u32
//     }
//
//     pub fn get_meta_key(&self) -> u32 {
//         (self.long_key as u32) & 0xFFFFFFFF
//     }
//
//     pub fn get_data_id(&self) -> u32 {
//         (self.long_key >> DATA_ID_SHR) as u32 & DATA_ID_BITS
//     }
//
//     pub fn get_data_number(&self) -> u32 {
//         (self.long_key >> DATA_NUMBER_SHR) as u32 & DATA_NUMBER_BITS
//     }
//
//     pub fn get_data_count(&self) -> u32 {
//         (self.long_key >> DATA_COUNT_SHR) as u32 & DATA_COUNT_BITS
//     }
//
//     pub fn get_data_type(&self) -> u32 {
//         (self.long_key >> DATA_TYPE_SHR) as u32 & DATA_TYPE_BITS
//     }
//
//     pub fn get_file_id(&self) -> u32 {
//         (self.long_key >> FILE_ID_SHR) as u32 & FILE_ID_BITS
//     }
//
//     pub fn get_file_number(&self) -> u32 {
//         (self.long_key >> FILE_NUMBER_SHR) as u32 & FILE_NUMBER_BITS
//     }
//
//     pub fn get_file_index(&self) -> usize {
//         ((self.long_key >> FILE_INDEX_SHR) as u32 & FILE_INDEX_BITS) as usize
//     }
// }