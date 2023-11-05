use crate::cache::CacheKey;
use crate::easing::Easing;


#[derive(Default)]
pub struct SpriteState {
    // is_current: bool,
    pub id: u32,
    pub name: String,
    pub map_x: i32,
    pub map_y: i32,
    pub point_x: f32,
    pub point_y: f32,
    pub level: u16,
    pub health: u16,
    pub power: u16,
    pub sex: u8,
    pub job: u8,
    pub dir: u8,
    pub hair: CacheKey,
    pub dress: CacheKey,
    pub weapon: CacheKey,
    pub effect: CacheKey,
    pub weapon_effect: CacheKey,
    pub state: u8,
    pub easing: Easing<f32>
}

impl SpriteState {


    pub fn move_tile(&mut self, map_x: i32, map_y: i32) {
        self.map_x = map_x;
        self.map_y = map_y;
    }

    pub fn dir(&mut self, dir: u8) {
        self.dir = dir;
    }

    pub fn state(&mut self, state: u8) {
        self.state = state;
    }

    pub fn easing(&mut self, duration: f64) {
        self.easing.advance(duration);
    }

    pub fn dress(&self) -> CacheKey {
        self.dress.as_inc_index(self.easing.now() as u32)
    }

    pub fn hair(&self) -> CacheKey {
        self.hair.as_inc_index(self.easing.now() as u32)
    }

    pub fn effect(&self) -> CacheKey {
        self.effect.as_inc_index(self.easing.now() as u32)
    }

    pub fn weapon(&self) -> CacheKey {
        self.weapon.as_inc_index(self.easing.now() as u32)
    }
    pub fn weapon_effect(&self) -> CacheKey {
        self.weapon_effect.as_inc_index(self.easing.now() as u32)
    }
}