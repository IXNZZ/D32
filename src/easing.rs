

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

}

fn angle(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    (dst_y - src_y).atan2(dst_x - src_x) * 57.295776
}

fn sharing(angle: f32, sharing: f32) -> f32 {
    let angle = angle + 90.0 + 360.0 / 2.0 / sharing;
    let angle = angle + if angle < 0. { 360.0 } else { 0. };
    ((angle) / (360. / sharing) + 1.0).floor()
}

pub fn angle8(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    // let angle = angle(src, dst);
    return sharing(angle(src_x, src_y, dst_x, dst_y), 8.);
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
