use ggez::Context;
use ggez::graphics::Canvas;
use crate::state::State;

pub mod map;

pub enum Layer {
    MapTile,
    MapMiddle,
    MapObjects,
    State,
}

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer) {

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