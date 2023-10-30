use std::path::PathBuf;
use keyframe::mint::Point2;
use tracing::error;
use crate::asset;
use crate::asset::Tile;
use crate::easing::{Easing};

// #[derive(Default)]
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
    pub(crate) easing: Easing<Point2<f32>>
}

impl MapState {

    pub fn new(base_dir: &PathBuf) -> Self {
        Self {
            map_dir: base_dir.join("map"),
            map_id: 0,
            map_name: String::new(),
            map_title: String::new(),
            map_data_id: 0,
            map_data_number: 0,
            map_width: 0,
            map_height: 0,
            sprite_tile_x: 0,
            sprite_tile_y: 0,
            sprite_abs_x: 0.,
            sprite_abs_y: 0.,
            tiles: Vec::new(),
            reload: false,
            easing: Easing::new(Point2 {x: 0.0, y: 0.0}, Point2 {x: 0.0, y: 0.0}, 1.0)
        }
    }
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

    pub fn move_by_pixel(&mut self, abs_x: f32, abs_y: f32) {
        self.sprite_abs_x = abs_x;
        self.sprite_abs_y = abs_y;
        let tile_x = (self.sprite_abs_x / 48.) as i32;
        let tile_y = (self.sprite_abs_y / 32.) as i32;
        if tile_x != self.sprite_tile_x || tile_y != self.sprite_tile_y {
            self.sprite_tile_x = tile_x;
            self.sprite_tile_y = tile_y;
            self.reload = true;
        }
    }

    pub fn easing_by_point(&mut self, rel_offset_x: f32, rel_offset_y: f32) {
        let start = Point2 {x: self.sprite_abs_x, y: self.sprite_abs_y};
        let finish = Point2 {x: self.sprite_abs_x + rel_offset_x, y: self.sprite_abs_y + rel_offset_y};
        println!("start: {:?}, finish: {:?}", start, finish);
        self.easing = Easing::once_finish(start, finish, 1.);
    }

    pub fn walk_by_direction(&mut self, dir_for_8: u8) {
        match dir_for_8 {
            1 => { self.easing_by_point(0.0, -32.0) },
            2 => { self.easing_by_point(48.0, -32.0) },
            3 => { self.easing_by_point(48.0, 0.0) },
            4 => { self.easing_by_point(48.0, 32.0) },
            5 => { self.easing_by_point(0.0, 32.0) },
            6 => { self.easing_by_point(-48.0, 32.0) },
            7 => { self.easing_by_point(-48.0, 0.0) },
            8 => { self.easing_by_point(-48.0, -32.0) },
            _ => {}
        }
    }

    pub fn run_by_direction(&mut self, dir_for_8: u8) {
        match dir_for_8 {
            1 => { self.easing_by_point(0.0, -64.0) },
            2 => { self.easing_by_point(96.0, -64.0) },
            3 => { self.easing_by_point(96.0, 0.0) },
            4 => { self.easing_by_point(96.0, 64.0) },
            5 => { self.easing_by_point(0.0, 64.0) },
            6 => { self.easing_by_point(-96.0, 64.0) },
            7 => { self.easing_by_point(-96.0, 0.0) },
            8 => { self.easing_by_point(-96.0, -64.0) },
            _ => {}
        }
    }
}