use std::fmt::Debug;
use std::sync::Arc;
use ggez::Context;
use ggez::glam::{vec2};
use ggez::graphics::{Canvas, DrawParam, InstanceArray, Rect};
use crate::asset::Tile;
use crate::cache::{CacheDataKey, CacheKey, CacheMetaKey, ImageMeta, ImageValue};
use crate::component::{Controller, Draw, Layer};
use crate::state::State;

#[derive(Debug)]
pub struct MapTileSet {
    layer: i32,
    even: bool,
    tile: Tile,
    x: f32,
    y: f32,
    tile_x: i32,
    tile_y: i32,
    back_key: CacheKey,
    middle_key: CacheKey,
    object_key: CacheKey,
}

pub struct MapComponent {
    map_id: u32,
    max_tile_width: i32,
    max_tile_height: i32,
    current_tile_set: Vec<MapTileSet>,
}

impl MapComponent {

    pub fn new(state: &mut State, window_width: f32, window_height: f32) -> Self {
        Self {
            map_id: state.map.map_id,
            max_tile_width: window_width as i32 / 48 + 1,
            max_tile_height: window_height as i32 / 32 + 1,
            current_tile_set: Vec::new(),
        }
    }


    fn build_map_window(&mut self, state: &mut State, layer: i32) {
        let map = &mut state.map;
        let max_width = self.max_tile_width + 4;
        let max_height = self.max_tile_height + 12;
        let start_x = map.sprite_tile_x - self.max_tile_width / 2 - 2;
        let start_y = map.sprite_tile_y - self.max_tile_height / 2 - 2;
        let mut sets: Vec<MapTileSet> = Vec::new();
        for w in 0..max_width {
            for h in 0..max_height {
                if w + start_x < 0 || w + start_x >= map.map_height || h + start_y < 0 || h + start_y >= map.map_width {
                    continue;
                }
                let p = (w + start_x) * map.map_height + h + start_y;
                let tile = &map.tiles[p as usize];
                let even = (w + start_x) & 0x1 != 1 && (h + start_y) & 0x1 != 1;

                let back_idx = tile.back as u32 & 0x7FFF;
                let middle_idx = tile.middle as u32 & 0x7FFF;
                let object_idx = tile.objects as u32 & 0x7FFF;
                let back_idx = if back_idx > 0 { back_idx - 1 } else { 0 };
                let middle_idx = if middle_idx > 0 { middle_idx - 1 } else { 0 };
                let object_idx = if object_idx > 0 { object_idx - 1 } else { 0 };


                sets.push(MapTileSet {
                    layer: layer + (w + start_x) * 1024,
                    even,
                    tile: tile.clone(),
                    x: w as f32 * 48.,
                    y: h as f32 * 32.,
                    tile_x: w + start_x,
                    tile_y: h + start_y,
                    back_key: CacheKey::from(map.map_data_id, map.map_data_number + 0, 2, 1, 1, tile.back_idx as u32 + 1, back_idx),
                    middle_key: CacheKey::from(map.map_data_id, map.map_data_number + 1, 2, 1, 2, tile.middle_idx as u32 + 1, middle_idx),
                    object_key: CacheKey::from(map.map_data_id, map.map_data_number + 2, 2, 1, 3, tile.objects_idx as u32 + 1, object_idx),
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
        state.cache.load_keys(back_keys.as_slice());
        state.cache.load_keys(middle_keys.as_slice());
        state.cache.load_keys(object_keys.as_slice());
        self.current_tile_set = sets;
    }

    pub fn draw_tile(&mut self, ctx: &mut Context, state: &mut State, canvas: &mut Canvas) {
        let map = &mut state.map;
        let back_data_key = CacheKey::build_data_key(map.map_data_id, map.map_data_number, 2);

        let filter = |value: &Arc<ImageValue>, tile: &MapTileSet| -> bool {
            tile.even && value.meta(tile.back_key.get_meta_key()).is_some()
        };
        let meta_key = |tile: &MapTileSet| -> CacheMetaKey {
            tile.back_key.get_meta_key()
        };
        let offset = |_meta: &ImageMeta| -> f32 {
            0.
        };
        self.draw_map(ctx, state, canvas, back_data_key, filter, meta_key, offset);

    }

    pub fn draw_middle(&mut self, ctx: &mut Context, state: &mut State, canvas: &mut Canvas) {
        let map = &mut state.map;
        let middle_data_key = CacheKey::build_data_key(map.map_data_id, map.map_data_number + 1, 2);

        let filter = |value: &Arc<ImageValue>, tile: &MapTileSet| -> bool {
            value.meta(tile.middle_key.get_meta_key()).is_some()
        };
        let meta_key = |tile: &MapTileSet| -> CacheMetaKey {
            tile.middle_key.get_meta_key()
        };
        let offset = |_meta: &ImageMeta| -> f32 {
            0.
        };
        self.draw_map(ctx, state, canvas, middle_data_key, filter, meta_key, offset);
    }

    pub fn draw_objects(&mut self, ctx: &mut Context, state: &mut State, canvas: &mut Canvas) {
        let map = &mut state.map;
        let object_data_key = CacheKey::build_data_key(map.map_data_id, map.map_data_number + 2, 2);

        let filter = |value: &Arc<ImageValue>, tile: &MapTileSet| -> bool {
            value.meta(tile.object_key.get_meta_key()).is_some()
        };
        let meta = |tile: &MapTileSet| -> CacheMetaKey {
            tile.object_key.get_meta_key()
        };
        let offset = |meta: &ImageMeta| -> f32 {
            meta.height as f32
        };
        self.draw_map(ctx, state, canvas, object_data_key, filter, meta, offset);
    }

    pub fn draw_map<F, M, S>(&mut self, ctx: &mut Context, state: &mut State, canvas: &mut Canvas, data_key: CacheDataKey, filter: F, meta_key: M, offset: S)
        where F: Fn(&Arc<ImageValue>, &MapTileSet) -> bool,
              M: Fn(&MapTileSet) -> CacheMetaKey,
        S: Fn(&ImageMeta) -> f32,
    {
        let map = &mut state.map;
        let cache = &mut state.cache;
        let rel_offset_x = (map.sprite_abs_x as i32 % 48) as f32;
        let rel_offset_y = (map.sprite_abs_y as i32 % 32) as f32;
        let dest = DrawParam::default().dest(vec2(-3. * 48., -3. * 32.));
        if let Some(value) = cache.get(&data_key) {
            let mut array = InstanceArray::new(ctx, value.image());
            let image_width = value.image().width() as f32;
            let image_height = value.image().height() as f32;
            array.set(self.current_tile_set
                .iter()
                .filter(|t|filter(&value, *t))
                .map(|t|{
                    let meta = value.meta(meta_key(t)).unwrap();
                    DrawParam::default().src(Rect::new(meta.src_x / image_width, meta.src_y / image_height, meta.width as f32 / image_width, meta.height as f32 / image_height))
                        .dest(vec2(meta.offset_x + t.x + rel_offset_x, meta.offset_y + t.y + rel_offset_y - offset(meta)))
                }));
            canvas.draw(&array, dest);
        }
    }


}

impl Draw for MapComponent {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer) {
        match layer {
            Layer::MapTile => {self.draw_tile(ctx, state, canvas)},
            Layer::MapMiddle => {self.draw_middle(ctx, state, canvas)},
            Layer::MapObjects => {self.draw_objects(ctx, state, canvas)},
            _ => {}
        }
    }
}

impl Controller for MapComponent {
    fn update(&mut self, ctx: &mut Context, state: &mut State) {
        if state.map.reload {
            state.map.reload = false;
            self.build_map_window(state, 255)
        }
    }
}