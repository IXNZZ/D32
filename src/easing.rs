

use keyframe::{AnimationSequence, CanTween, keyframes};
use keyframe_derive::CanTween;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EasingStatus {
    Ready,
    Run,
    Pause,
    PauseStart,
    PauseFinish,
    Stop,
}

impl Default for EasingStatus {
    fn default() -> Self {
        EasingStatus::Ready
    }
}

#[derive(CanTween, Clone, Default, Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

#[derive(CanTween, Clone, Default, Debug)]
pub struct Rect2 {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Default)]
pub struct Easing<T: CanTween + Clone + Default> {
    status: EasingStatus,
    sequence: AnimationSequence<T>,
    timestamp: f64,
}

impl<T: CanTween + Clone + Default> Easing<T> {
    pub fn status(&self) -> EasingStatus {
        self.status
    }

    pub fn run(&mut self) {
        self.status = EasingStatus::Run;
    }

    pub fn pause(&mut self) {
        self.status = EasingStatus::Pause;
    }

    pub fn pause_to_start(&mut self) {
        self.status = EasingStatus::PauseStart;
    }

    pub fn pause_to_finish(&mut self) {
        self.status = EasingStatus::PauseFinish;
    }

    pub fn stop(&mut self) {
        self.status = EasingStatus::Stop;
    }

    pub fn ready(&mut self) {
        self.status = EasingStatus::Ready;
        self.sequence.advance_to(0.0);
    }

    pub fn advance(&mut self, duration: f64) -> bool {

        if self.status == EasingStatus::Run {
            self.sequence.advance_and_maybe_wrap(duration);
        } else if self.status == EasingStatus::PauseStart && self.sequence.advance_and_maybe_wrap(duration) {
            self.sequence.advance_to(0.0);
            self.pause();
            return true;
        } else if self.status == EasingStatus::PauseFinish {
            if self.sequence.advance_by(duration) > 0.0 {
                self.sequence.advance_to(self.timestamp);
                self.pause();
                return true;
            }
        }
        return false;
    }

    pub fn new(start: T, finish: T, time: f64) -> Self {
        let sequence = keyframes![
            (start, 0., keyframe::functions::Linear),
            (finish, time, keyframe::functions::Linear)
        ];
        Self {
            status: EasingStatus::Ready,
            sequence,
            timestamp: time,
        }
    }

    pub fn wrap_run(start: T, finish: T, time: f64) -> Self {
        let mut easing = Easing::new(start, finish, time);
        easing.run();
        easing
    }

    pub fn once_start(start: T, finish: T, time: f64) -> Self {
        let mut easing = Easing::new(start, finish, time);
        easing.pause_to_start();
        easing
    }

    pub fn once_finish(start: T, finish: T, time: f64) -> Self {
        let mut easing = Easing::new(start, finish, time);
        easing.pause_to_finish();
        easing
    }



    pub fn now(&self) -> T {
        self.sequence.now()
    }

    // pub fn finish(&self) -> T {
    //     // self.sequence.
    // }

}

fn angle(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    (dst_y - src_y).atan2(dst_x - src_x) * 57.295776
}

fn sharing(angle: f32, sharing: f32) -> f32 {
    let angle = angle + 90.0 + 360.0 / 2.0 / sharing;
    let angle = angle + if angle < 0. { 360.0 } else { 0. };
    ((angle) / (360. / sharing) + 1.0).floor()
}

pub fn angle8(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> Direction {
    // let angle = angle(src, dst);
    return Direction::from(sharing(angle(src_x, src_y, dst_x, dst_y), 8.) as u32);
}

pub fn angle12(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    return sharing(angle(src_x, src_y, dst_x, dst_y), 12.);
}

pub fn angle16(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    return sharing(angle(src_x, src_y, dst_x, dst_y), 16.);
}

pub fn distance(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    ((dst_x - src_x).abs().powi(2) + (dst_y - src_y).abs().powi(2)).sqrt()
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    North, //北
    Northeast, //东北
    East, //东
    Southeast, // 东南
    South, // 南
    Southwest, // 西南
    West, //西
    Northwest // 西北
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}


impl Direction {

    pub fn from(dir_of_8: u32) -> Self {
        match dir_of_8 {
            1 => { Direction::North },
            2 => { Direction::Northeast },
            3 => { Direction::East },
            4 => { Direction::Southeast },
            5 => { Direction::South },
            6 => { Direction::Southwest },
            7 => { Direction::West },
            8 => { Direction::Northwest },
            _ => {Direction::North}
        }
    }
    pub fn offset(&self) -> f32 {
        match self {
            Direction::North => {0.0}
            Direction::Northeast => {1.0}
            Direction::East => {2.0}
            Direction::Southeast => {3.0}
            Direction::South => {4.0}
            Direction::Southwest => {5.0}
            Direction::West => {6.0}
            Direction::Northwest => {7.0}
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PlayerAction {
    Stand, //站立
    Walk, //步行
    Run, //跑步
    WarMode, //准备攻击
    Hit, //攻击
    HeavyHit, //重要的攻击
    BigHit, // 主要的攻击
    // FireHitReady, //魔法
    Spell, //魔法
    SitDown,
    Damage, //受到伤害
    Die,
}

impl Default for PlayerAction {
    fn default() -> Self {
        PlayerAction::Stand
    }
}

impl PlayerAction {

    pub fn new_frame(&self) -> Easing<f32> {
        Easing::new(0.0, self.step() - 0.001, self.time())
    }

    pub fn effect_frame(&self) -> Easing<f32> {
        Easing::new(0.0, self.effect_step() - 0.001, self.time())
    }

    fn time(&self) -> f64 {
        match self {
            PlayerAction::Stand => { 1.5 }
            PlayerAction::Walk => { 0.8 }
            PlayerAction::Run => { 1.0 }
            PlayerAction::WarMode => {0.5}
            PlayerAction::Hit => {0.7}
            PlayerAction::HeavyHit => {0.75}
            PlayerAction::BigHit => {0.8}
            // PlayerAction::FireHitReady => {6.0}
            PlayerAction::Spell => {0.8}
            PlayerAction::SitDown => {1.2}
            PlayerAction::Damage => {0.7}
            PlayerAction::Die => {1.0}
        }
    }
    pub fn step(&self) -> f32 {
        match self {
            PlayerAction::Stand => {4.0}
            PlayerAction::Walk => {6.0}
            PlayerAction::Run => {6.0}
            PlayerAction::WarMode => {1.0}
            PlayerAction::Hit => {6.0}
            PlayerAction::HeavyHit => {6.0}
            PlayerAction::BigHit => {8.0}
            // PlayerAction::FireHitReady => {6.0}
            PlayerAction::Spell => {6.0}
            PlayerAction::SitDown => {2.0}
            PlayerAction::Damage => {3.0}
            PlayerAction::Die => {4.0}
        }
    }

    pub fn effect_step(&self) -> f32 {
        match self {
            PlayerAction::Stand => {8.0}
            PlayerAction::Walk => {6.0}
            PlayerAction::Run => {6.0}
            PlayerAction::WarMode => {1.0}
            PlayerAction::Hit => {6.0}
            PlayerAction::HeavyHit => {6.0}
            PlayerAction::BigHit => {8.0}
            // PlayerAction::FireHitReady => {6.0}
            PlayerAction::Spell => {6.0}
            PlayerAction::SitDown => {2.0}
            PlayerAction::Damage => {3.0}
            PlayerAction::Die => {4.0}
        }
    }

    pub fn jump(&self) -> f32 {
        match self {
            PlayerAction::Stand => {0.0}
            PlayerAction::Walk => {32.0}
            PlayerAction::Run => {80.0}
            PlayerAction::WarMode => {128.0}
            PlayerAction::Hit => {136.0}
            PlayerAction::HeavyHit => {184.0}
            PlayerAction::BigHit => {232.0}
            // PlayerAction::FireHitReady => {296.0}
            PlayerAction::Spell => {296.0}
            PlayerAction::SitDown => {344.0}
            PlayerAction::Damage => {360.0}
            PlayerAction::Die => {384.0}
        }
    }

    pub fn effect_jump(&self) -> f32 {
        match self {
            PlayerAction::Stand => {0.0}
            PlayerAction::Walk => {64.0}
            PlayerAction::Run => {80.0 + 32.0}
            PlayerAction::WarMode => {128.0 + 32.0}
            PlayerAction::Hit => {136.0 + 32.0}
            PlayerAction::HeavyHit => {184.0 + 32.0}
            PlayerAction::BigHit => {232.0 + 32.0}
            PlayerAction::Spell => {296.0 + 32.0}
            PlayerAction::SitDown => {344.0 + 32.0}
            PlayerAction::Damage => {360.0 + 32.0}
            PlayerAction::Die => {384.0 + 32.0}
        }
    }
}

#[derive(Default)]
pub struct PlayerAnimation {
    state: PlayerAction,
    dir: Direction,
    // dress: Easing<f32>,
    frame: Easing<f32>,
    effect: Easing<f32>,
}

impl PlayerAnimation {

    pub fn new(state: PlayerAction, dir: Direction) -> Self {
        // let dress: Easing<f32> = state.new_frame();
        let frame: Easing<f32> = state.new_frame();
        let effect: Easing<f32> = state.effect_frame();
        Self {state, dir, frame, effect}
    }

    pub fn advance(&mut self, duration: f64) {
        // self.dress.advance(duration);
        // println!("easing: {:?}", self.frame.status);
        self.frame.advance(duration);
        self.effect.advance(duration);
    }

    pub fn state(&mut self, state: PlayerAction) {
        // self.dress = state.new_frame();
        self.frame = state.new_frame();
        self.effect = state.effect_frame();
        self.state = state;
    }

    pub fn status(&mut self, status: EasingStatus) {
        // self.dress.status = status;
        self.frame.status = status;
        self.effect.status = status;
    }

    pub fn dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn now(&self) -> f32 {
        let count = self.frame.now();
        self.state.step() * self.dir.offset() + self.state.jump() + count + 1.0
    }

    pub fn effect(&self) -> f32 {
        let count = self.effect.now();
        self.state.effect_step() * self.dir.offset() + self.state.effect_jump() + count + 1.0
    }
}
