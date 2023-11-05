use ggez::Context;
use ggez::event::MouseButton;
use ggez::graphics::Canvas;
use crate::state::State;

pub mod map;
pub mod debug;
pub mod sprite;

pub enum Layer {
    MapTile,
    MapMiddle,
    MapObjects,
    State,

    Debug,
}

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer);
}

pub trait Controller {
    fn update(&mut self, ctx: &mut Context, state: &mut State);
}

pub trait Event {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32, state: &mut State) {

    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) {
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, _state: &mut State) {

    }
}

pub trait Initializer {

}