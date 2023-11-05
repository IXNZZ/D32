use ggez::Context;
use ggez::graphics::Canvas;
use crate::component::{Controller, Draw, Layer};
use crate::state::State;

pub struct SpriteComponent{

}

impl SpriteComponent {

    pub fn new(_ctx: &mut Context, state: &mut State) -> Self {
        // test code
        state.sprite.id = 1;



        // end code
        Self {}
    }

}

impl Controller for SpriteComponent {
    fn update(&mut self, ctx: &mut Context, state: &mut State) {
        state.sprite.easing(ctx.time.delta().as_secs_f64());
    }
}

impl Draw for SpriteComponent {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer) {

    }
}