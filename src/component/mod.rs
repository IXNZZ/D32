use ggez::Context;
use crate::state::State;

pub mod map;

pub enum Layer {
    Map(i32),

}

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context, state: &mut State) {

    }
}

pub trait Controller {
    fn update(&mut self, ctx: &mut Context, state: &mut State) {

    }
}

pub trait Event {

}

pub trait Initializer {

}