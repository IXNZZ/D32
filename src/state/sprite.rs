use crate::cache::CacheKey;

#[derive(Debug, Default)]
pub struct SpriteState {
    id: u32,
    name: String,
    curr_x: i32,
    curr_y: i32,
    sex: u8,
    job: u8,
    dir: u8,
    hair: CacheKey,
    dress: CacheKey,
    weapon: CacheKey,
    effect: CacheKey,
    weapon_effect: CacheKey,
    state: u8,
}

impl SpriteState {
    pub fn walk(&mut self, new_x: i32, new_y: i32) {

    }

    pub fn run(&mut self, new_x: i32, new_y: i32) {

    }

    pub fn dir(&mut self, dir: u8) {

    }
}