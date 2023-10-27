use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use ggez::Context;
use ggez::glam::{vec2};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, InstanceArray, Mesh, Rect, ScreenImage, StrokeOptions, Text};
use tracing::error;
use crate::{asset};
use crate::asset::Tile;
use crate::cache::{CacheKey, ImageCache};

#[derive(Debug)]
pub struct MapTileSet {
    layer: i32,
    even: bool,
    tile: Tile,
    x: f32,
    y: f32,
    back_key: CacheKey,
    middle_key: CacheKey,
    object_key: CacheKey,
}

pub struct MapDraw {
    map_dir: PathBuf,
    map_name: String,
    map_title: String,
    data_id: u32,
    data_number: u32,
    max_tile_width: i32,
    max_tile_height: i32,
    tile_width: i32,
    tile_height: i32,
    current_tile_x: i32,
    current_tile_y: i32,
    absolute_offset_x: f32,
    absolute_offset_y: f32,
    tiles: Vec<Tile>,
    reload: bool,
    current_tile_set: Vec<MapTileSet>,
}

impl MapDraw {

    pub fn new(base_dir: &Path, data_id: u32, data_number: u32, name: &str, title: &str, window_width: f32, window_height: f32) -> Self {
        let mut this = Self {
            map_dir: base_dir.join("map"),
            map_name: String::from(name),
            map_title: String::from(title),
            data_id,
            data_number,
            max_tile_width: window_width as i32 / 48,
            max_tile_height: window_height as i32 / 32,
            tile_width: 0,
            tile_height: 0,
            current_tile_x: 0,
            current_tile_y: 0,
            absolute_offset_x: 0.,
            absolute_offset_y: 0.,
            tiles: Vec::new(),
            reload: true,
            current_tile_set: Vec::new(),
        };
        this.reload_map_data();
        this
    }

    fn reload_map_data(&mut self) {
        if let Some(data) = asset::read_map_file(self.map_dir.join(&self.map_name).with_extension("map")) {
            self.tile_width = data.width as i32;
            self.tile_height = data.height as i32;
            self.tiles = data.tiles;
        } else {
            error!("未找到地图: {}, {}", self.map_name, self.map_title);
        }

    }

    pub fn reload_map(&mut self, data_id: u32, data_number: u32, name: &str, title: &str, tile_x: i32, tile_y: i32, rel_offset_x: i32, rel_offset_y: i32) {
        self.map_name = String::from(name);
        self.map_title = String::from(title);
        self.data_id = data_id;
        self.data_number = data_number;
        self.reload_map_data();
        self.jump_by_tile(tile_x, tile_y, rel_offset_x, rel_offset_y);
        self.reload = true;
    }

    pub fn jump_by_tile(&mut self, tile_x: i32, tile_y: i32, rel_offset_x: i32, rel_offset_y: i32) {
        self.current_tile_x = tile_x;
        self.current_tile_y = tile_y;
        if tile_x > self.tile_width {
            self.current_tile_x = self.tile_width;
        }
        if tile_y > self.tile_height {
            self.current_tile_y = self.tile_height;
        }
        self.absolute_offset_x = ((self.current_tile_x - 1) * 48 + rel_offset_x) as f32;
        self.absolute_offset_y = ((self.current_tile_y - 1) * 32 + rel_offset_y) as f32;
        self.reload = true;
    }

    pub fn move_by_pixel(&mut self, rel_offset_x: f32, rel_offset_y: f32) {
        self.absolute_offset_x += rel_offset_x;
        self.absolute_offset_y += rel_offset_y;
        let tile_x = (self.absolute_offset_x / 48.) as i32;
        let tile_y = (self.absolute_offset_y / 32.) as i32;
        if tile_x != self.current_tile_x || tile_y != self.current_tile_y {
            self.current_tile_x = tile_x;
            self.current_tile_y = tile_y;
            self.reload = true;
        }
    }

    pub fn update_move_pixel(&mut self, time: f64) {

    }

    fn build_map_window(&mut self, cache: &mut ImageCache, layer: i32) {
        let max_width = self.max_tile_width + 4;
        let max_height = self.max_tile_height + 12;
        let start_x = self.current_tile_x - self.max_tile_width / 2 - 2;
        let start_y = self.current_tile_y - self.max_tile_height / 2 - 2;
        let mut sets: Vec<MapTileSet> = Vec::new();
        // println!("max w: {}, h: {}, start x: {}, y: {}", max_width, max_height, start_x, start_y);
        for w in 0..max_width {
            for h in 0..max_height {
                if w + start_x < 0 || w + start_x >= self.tile_height || h + start_y < 0 || h + start_y >= self.tile_width {
                    continue;
                }
                let p = (w + start_x) * self.tile_height + h + start_y;
                let tile = &self.tiles[p as usize];
                let even = (w + start_x) & 0x1 != 1 && (h + start_y) & 0x1 != 1;
                // cache::build_cache_key()

                // println!("even: {even}, p: {p}, w: {w}, h: {h}, start_x: {start_x}, start_y: {start_y}, tile: {:?}", tile);

                let back_idx = (tile.back as u32 & 0x7FFF);
                let middle_idx = (tile.middle as u32 & 0x7FFF);
                let object_idx = (tile.objects as u32 & 0x7FFF);
                let back_idx = if back_idx > 0 { back_idx - 1 } else { 0 };
                let middle_idx = if middle_idx > 0 { middle_idx - 1 } else { 0 };
                let object_idx = if object_idx > 0 { object_idx - 1 } else { 0 };


                sets.push(MapTileSet {
                    layer: layer + (w + start_x) * 1024,
                    even,
                    tile: tile.clone(),
                    x: w as f32 * 48.,
                    y: h as f32 * 32.,
                    back_key: CacheKey::from(self.data_id, self.data_number + 0, 2, 1, 1, tile.back_idx as u32 + 1, back_idx),
                    middle_key: CacheKey::from(self.data_id, self.data_number + 1, 2, 1, 2, tile.middle_idx as u32 + 1, middle_idx),
                    object_key: CacheKey::from(self.data_id, self.data_number + 2, 2, 1, 3, tile.objects_idx as u32 + 1, object_idx),
                })
            }
        }
        let back_keys = sets.iter().filter(|x| x.even && (x.tile.back & 0x7FFF) > 0).map(|t| {
            t.back_key
        }).collect::<Vec<CacheKey>>();
        let middle_keys = sets.iter().filter(|x| (x.tile.middle & 0x7FFF) > 0).map(|t| {
            t.middle_key
        }).collect::<Vec<CacheKey>>();
        let object_keys = sets.iter().filter(|x| (x.tile.objects & 0x7FFF) > 0).map(|t| {
            t.object_key
        }).collect::<Vec<CacheKey>>();
        // println!("back keys len: {}", back_keys.len());
        cache.load_keys(back_keys.as_slice());
        cache.load_keys(middle_keys.as_slice());
        cache.load_keys(object_keys.as_slice());
        self.current_tile_set = sets;
    }

    pub fn draw_tile(&mut self, canvas: &mut Canvas, ctx: &mut Context, cache: &mut ImageCache, layer: i32) {

        if self.reload {
            self.reload = false;
            self.build_map_window(cache, layer);
        }

        let time = Instant::now();
        // let image = ScreenImage::new(ctx, None, 1., 1., 1).image(ctx);
        // let mut text_canvas = Canvas::from_image(ctx, image.clone(), None);


        let rel_offset_x = (self.absolute_offset_x as i32 % 48) as f32;
        let rel_offset_y = (self.absolute_offset_y as i32 % 32) as f32;
        let back_data_key = CacheKey::build_data_key(self.data_id, self.data_number, 2);

        let dest = DrawParam::default().dest(vec2(-3. * 48., -3. * 32.));

        if let Some(value) = cache.get(ctx, &back_data_key) {
            // println!("value: {}", value.)
            let image_width = value.image().width() as f32;
            let image_height = value.image().height() as f32;
            let mut array = InstanceArray::new(ctx, value.image());
            array.set(self.current_tile_set
                .iter()
                .filter(|t|t.even)
                .filter(|t|{
                    value.meta(t.back_key.get_meta_key()).is_some()
                })
                .map(|t|{
                    let meta = value.meta(t.back_key.get_meta_key()).unwrap();
                    DrawParam::default().src(Rect::new(meta.src_x / image_width, meta.src_y / image_height, meta.width as f32 / image_width, meta.height as f32 / image_height))
                        .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y))
            }));
            // text_canvas.finish(ctx).unwrap();
            canvas.draw(&array, dest);
            // canvas.draw(&image, DrawParam::default());
            // canvas.draw(&value.image(), DrawParam::default());
        }
        // println!("draw back: {:?}", time.elapsed());
        let middle_data_key = CacheKey::build_data_key(self.data_id, self.data_number + 1, 2);
        if let Some(value) = cache.get(ctx, &middle_data_key) {
            let mut array = InstanceArray::new(ctx, value.image());
            let image_width = value.image().width() as f32;
            let image_height = value.image().height() as f32;
            array.set(self.current_tile_set
                .iter()
                .filter(|t|value.meta(t.middle_key.get_meta_key()).is_some())
                .map(|t|{
                    let meta = value.meta(t.middle_key.get_meta_key()).unwrap();
                    DrawParam::default().src(Rect::new(meta.src_x / image_width, meta.src_y / image_height, meta.width as f32 / image_width, meta.height as f32 / image_height))
                        .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y))
                }));
            canvas.draw(&array, dest);
        }
        // println!("draw middle: {:?}", time.elapsed());




    }

    pub fn draw_objects(&mut self, ctx: &mut Context, canvas: &mut Canvas, cache: &mut ImageCache) {
        let rel_offset_x = (self.absolute_offset_x as i32 % 48) as f32;
        let rel_offset_y = (self.absolute_offset_y as i32 % 32) as f32;
        let dest = DrawParam::default().dest(vec2(-3. * 48., -3. * 32.));
        let object_data_key = CacheKey::build_data_key(self.data_id, self.data_number + 2, 2);
        if let Some(value) = cache.get(ctx, &object_data_key) {
            let mut array = InstanceArray::new(ctx, value.image());
            let image_width = value.image().width() as f32;
            let image_height = value.image().height() as f32;
            array.set(self.current_tile_set
                .iter()
                .filter(|t|value.meta(t.object_key.get_meta_key()).is_some())
                .map(|t|{
                    let meta = value.meta(t.object_key.get_meta_key()).unwrap();
                    DrawParam::default().src(Rect::new(meta.src_x / image_width, meta.src_y / image_height, meta.width as f32 / image_width, meta.height as f32 / image_height))
                        .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y - meta.height as f32))
                }));
            canvas.draw(&array, dest);
        }
    }


}