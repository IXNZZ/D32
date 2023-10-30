use std::path::PathBuf;
use tracing::error;
use crate::asset;
use crate::asset::Tile;

#[derive(Debug, Default)]
pub struct MapState {
    pub(crate) map_dir: PathBuf,
    pub(crate) map_id: u32,
    pub(crate) map_name: String,
    pub(crate) map_title: String,
    pub(crate) map_data_id: u32,
    pub(crate) map_data_number: u32,
    pub(crate) map_width: i32,
    pub(crate) map_height: i32,
    pub(crate) sprite_tile_x: i32,
    pub(crate) sprite_tile_y: i32,
    pub(crate) sprite_abs_x: f32,
    pub(crate) sprite_abs_y: f32,
    pub(crate) tiles: Vec<Tile>,
    pub(crate) reload: bool,
}

impl MapState {
    pub fn load_map_file(&mut self) {
        if let Some(data) = asset::read_map_file(self.map_dir.join(&self.map_name).with_extension("map")) {
            self.map_width = data.width as i32;
            self.map_height = data.height as i32;
            self.tiles = data.tiles;
        } else {
            error!("未找到地图: {}", self.map_name);
        }
    }

    pub fn reload_map(&mut self, map_id: u32, data_id: u32, data_number: u32, name: &str, title: &str, tile_x: i32, tile_y: i32, rel_offset_x: i32, rel_offset_y: i32) {
        self.map_id = map_id;
        self.map_name = String::from(name);
        self.map_title = String::from(title);
        self.map_data_id = data_id;
        self.map_data_number = data_number;
        self.load_map_file();
        self.move_by_tile(tile_x, tile_y, rel_offset_x, rel_offset_y);
        // self.current_tile_set = Vec::new();
        self.reload = true;
    }

    pub fn move_by_tile(&mut self, tile_x: i32, tile_y: i32, rel_offset_x: i32, rel_offset_y: i32) {
        self.sprite_tile_x = tile_x;
        self.sprite_tile_y = tile_y;
        if tile_x > self.map_width {
            self.sprite_tile_x = self.map_width - 1;
        }
        if tile_y > self.map_height {
            self.sprite_tile_y = self.map_height - 1;
        }
        self.sprite_abs_x = (self.sprite_tile_x * 48 + rel_offset_x) as f32;
        self.sprite_abs_y = (self.sprite_tile_y * 32 + rel_offset_y) as f32;
        self.reload = true;
    }

    pub fn move_by_pixel(&mut self, rel_offset_x: f32, rel_offset_y: f32) {
        self.sprite_abs_x += rel_offset_x;
        self.sprite_abs_y += rel_offset_y;
        let tile_x = (self.sprite_abs_x / 48.) as i32;
        let tile_y = (self.sprite_abs_y / 32.) as i32;
        if tile_x != self.sprite_tile_x || tile_y != self.sprite_tile_y {
            self.sprite_tile_x = tile_x;
            self.sprite_tile_y = tile_y;
            self.reload = true;
        }
    }
}