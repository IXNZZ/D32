use crate::cache::CacheKey;
use crate::easing::{Direction, EasingStatus, PlayerAction, PlayerAnimation};
use crate::state::State;


#[derive(Default)]
pub struct SpriteState {
    // is_current: bool,
    pub id: u32,
    pub name: String,
    pub map_x: i32,
    pub map_y: i32,
    pub abs_point_x: f32,
    pub abs_point_y: f32,
    pub level: u16,
    pub health: u16,
    pub power: u16,
    pub sex: u8,
    pub job: u8,
    pub dir: Direction,
    pub hair: CacheKey,
    pub dress: CacheKey,
    pub weapon: CacheKey,
    pub effect: CacheKey,
    pub weapon_effect: CacheKey,
    pub action: PlayerAction,
    pub easing: PlayerAnimation
}

impl SpriteState {

    pub fn new_test(state: &mut State) -> Self {
        let mut sprite = SpriteState {
            id: 1,
            name: "人物名称".to_string(),
            map_x: 333,
            map_y: 333,
            abs_point_x: 333.0 * 48.0,
            abs_point_y: 333.0 * 32.0,
            level: 0,
            health: 0,
            power: 0,
            sex: 0,
            job: 0,
            action: PlayerAction::Stand,
            dress: CacheKey::from(2, 4, 1, 416, 4, 0, 0),
            hair: CacheKey::from(2, 5, 1, 416, 5, 0, 832),
            effect: CacheKey::from(2, 7, 1, 600, 6, 0, 0),
            weapon: CacheKey::from(2, 6, 1, 416, 7, 0, 0),
            weapon_effect: CacheKey::from(2, 8, 1, 416, 8, 0, 0),
            dir: Direction::North,
            easing: PlayerAnimation::new(PlayerAction::Stand, Direction::North),
        };

        state.cache.load_key(sprite.dress);
        state.cache.load_key(sprite.hair);
        state.cache.load_key(sprite.weapon);
        state.cache.load_key(sprite.effect);
        state.cache.load_key(sprite.weapon_effect);

        sprite.easing.status(EasingStatus::Run);

        sprite
    }


    pub fn move_tile(&mut self, map_x: i32, map_y: i32) {
        self.map_x = map_x;
        self.map_y = map_y;
    }

    pub fn dir(&mut self, dir: Direction) {
        self.dir = dir;
        self.easing.dir(dir);
    }

    pub fn action(&mut self, action: PlayerAction) {
        self.action = action;
        self.easing.state(action);
        self.easing.status(EasingStatus::Run);
        if action == PlayerAction::Die {
            self.easing.status(EasingStatus::PauseFinish);
        }
    }

    pub fn easing(&mut self, duration: f64) {
        self.easing.advance(duration);
    }

    pub fn dress(&self) -> CacheKey {
        // println!("dress: {}, idx: {}, key: {:?}", self.easing.now());
        self.dress.as_inc_index(self.easing.now() as u32)
    }

    pub fn hair(&self) -> CacheKey {
        self.hair.as_inc_index(self.easing.now() as u32)
    }

    pub fn effect(&self) -> CacheKey {
        self.effect.as_inc_index(self.easing.effect() as u32)
    }

    pub fn weapon(&self) -> CacheKey {
        self.weapon.as_inc_index(self.easing.now() as u32)
    }
    pub fn weapon_effect(&self) -> CacheKey {
        self.weapon_effect.as_inc_index(self.easing.now() as u32)
    }
}