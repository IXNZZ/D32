use std::path::{PathBuf};
use crate::cache::ImageCache;

pub mod map;

#[derive(Debug, Clone)]
pub struct GameState {
    pub(crate) base_dir: PathBuf,
    pub(crate) scale_factor: f32,
    pub(crate) screen_size: (f32, f32),
    pub(crate) window_size: (f32, f32),
    pub(crate) center_point: (f32, f32),
}


