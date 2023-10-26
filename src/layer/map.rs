use std::arch::aarch64::float32x2_t;
use ggez::Context;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, DrawParam, InstanceArray, Rect};
use tracing::error;
use crate::{asset, cache_1};
use crate::asset::Tile;
use crate::cache::{CacheKey, ImageCache};
// use crate::cache_1::ImageCacheManager;

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

#[derive(Default)]
pub struct MapLayer {
    map_dir: String,
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

impl MapLayer {

    pub fn new(map_dir: &str, data_id: u32, data_number: u32, name: &str, title: &str, window_width: f32, window_height: f32) -> Self {
        let mut this = Self {
            map_dir: String::from(map_dir),
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
        if let Some(data) = asset::read_map_file(format!("{}/{}.map", self.map_dir, self.map_name).as_str()) {
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

    fn build_map_window(&mut self, cache: &mut ImageCache, layer: i32) {
        let max_width = self.max_tile_width + 4;
        let max_height = self.max_tile_height + 10;
        let start_x = self.current_tile_x - self.max_tile_width / 2 - 2;
        let start_y = self.current_tile_y - self.max_tile_height / 2 - 2;
        let mut sets: Vec<MapTileSet> = Vec::new();
        for w in 0..max_width {
            for h in 0..max_height {
                if w + start_x < 0 || w + start_x >= self.tile_height || h + start_y < 0 || h + start_y >= self.tile_width {
                    continue;
                }
                let p = (w + start_x) * self.tile_height + h + start_y;
                let tile = &self.tiles[p as usize];
                let even = (w + start_x) & 0x1 != 1 && (h + start_y) & 0x1 != 1;
                // cache::build_cache_key()

                println!("even: {even}, p: {p}, w: {w}, h: {h}, start_x: {start_x}, start_y: {start_y}, tile: {:?}", tile);

                sets.push(MapTileSet {
                    layer: layer + w,
                    even,
                    tile: tile.clone(),
                    x: w as f32 * 48.,
                    y: h as f32 * 32.,
                    back_key: CacheKey::from(self.data_id, self.data_number + 0, 2, 1, 1, tile.back_idx as u32 + 1, (tile.back & 0x7FFF) as u32),
                    middle_key: CacheKey::from(self.data_id, self.data_number + 1, 2, 1, 2, tile.middle_idx as u32 + 1, (tile.middle & 0x7FFF) as u32),
                    object_key: CacheKey::from(self.data_id, self.data_number + 2, 2, 1, 3, tile.objects_idx as u32 + 1, (tile.objects & 0x7FFF) as u32),
                })
            }
        }

        let back_keys = sets.iter().filter(|x| x.even && (x.tile.back & 0x7FFF) > 0).map(|t| {
            CacheKey::new(t.back_key.get_long_key() - 1)
        }).collect::<Vec<CacheKey>>();
        let middle_keys = sets.iter().filter(|x| (x.tile.middle & 0x7FFF) > 0).map(|t| {
            CacheKey::new(t.middle_key.get_long_key() - 1)
        }).collect::<Vec<CacheKey>>();
        let object_keys = sets.iter().filter(|x| (x.tile.objects & 0x7FFF) > 0).map(|t| {
            CacheKey::new(t.object_key.get_long_key() - 1)
        }).collect::<Vec<CacheKey>>();
        println!("back keys len: {}", back_keys.len());
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
        let rel_offset_x = (self.absolute_offset_x as i32 % 48) as f32;
        let rel_offset_y = (self.absolute_offset_y as i32 % 32) as f32;
        let back_data_key = CacheKey::build_data_key(self.data_id, self.data_number, 2);
        let middle_data_key = CacheKey::build_data_key(self.data_id, self.data_number + 1, 2);
        if let Some(value) = cache.get(ctx, &back_data_key) {
            // println!("value: {}", value.)
            let mut array = InstanceArray::new(ctx, value.image());
            array.set(self.current_tile_set
                .iter()
                .filter(|t|t.even)
                .filter(|t|value.meta(t.back_key.get_meta_key()).is_some())
                .map(|t|{
                    let meta = value.meta(t.back_key.get_meta_key()).unwrap();
                    println!("x: {}, y: {}, meta: {:?}", meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y, meta);
                    DrawParam::default().src(Rect::new(meta.src_x, meta.src_y, meta.width as f32, meta.height as f32))
                        .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y)).z(0xFF)
            }));
            canvas.draw(&array, DrawParam::default());
            // canvas.draw(&value.image(), DrawParam::default());
        }

        // if let Some(value) = cache.get(ctx, &middle_data_key) {
        //     let mut array = InstanceArray::new(ctx, value.image());
        //     array.set(self.current_tile_set
        //         .iter()
        //         .filter(|t|value.meta(t.middle_key.get_meta_key()).is_some())
        //         .map(|t|{
        //             let meta = value.meta(t.middle_key.get_meta_key()).unwrap();
        //             DrawParam::default().src(Rect::new(meta.src_x, meta.src_y, meta.width as f32, meta.height as f32))
        //                 .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y))
        //         }));
        //     canvas.draw(&array, DrawParam::default());
        // }
    }
}