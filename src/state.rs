use std::path::{PathBuf};
use ggez::Context;
use crate::cache::ImageCache;

#[derive(Debug, Default)]
pub struct MapState {
    pub(crate) map_id: u32,
    pub(crate) map_name: String,
    pub(crate) map_title: String,
    pub(crate) map_data_id: u32,
    pub(crate) map_data_number: u32,
    pub(crate) map_max_width: u32,
    pub(crate) map_max_height: u32,
    pub(crate) sprite_tile_x: i32,
    pub(crate) sprite_tile_y: i32,
    pub(crate) sprite_abs_x: f32,
    pub(crate) sprite_abs_y: f32,
}

pub struct SpriteState {

}

pub struct State {
    pub(crate) base_dir: PathBuf,
    pub(crate) scale_factor: f32,
    pub(crate) screen_width: f32,
    pub(crate) screen_height: f32,
    pub(crate) window_width: f32,
    pub(crate) window_height: f32,
    pub(crate) center_x: f32,
    pub(crate) center_y: f32,
    pub(crate) cache: ImageCache,
    pub(crate) map: MapState,
}

impl State {

    pub fn new(base_dir: PathBuf, ctx: &mut Context) -> Self {
        let scale_factor = ctx.gfx.window().scale_factor();
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let monitor_size = ctx.gfx.window().current_monitor().unwrap().size();
        Self {
            base_dir: base_dir.clone(),
            scale_factor: scale_factor as f32,
            screen_width: monitor_size.width as f32,
            screen_height: monitor_size.height as f32,
            window_width,
            window_height,
            center_x: window_width / 2.,
            center_y: window_height / 2.,
            cache: ImageCache::new(base_dir),
            map: MapState::default()
        }
    }
}