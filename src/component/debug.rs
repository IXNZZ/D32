use ggez::Context;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Text};
use crate::component::{Draw, Event, Layer};
use crate::state::State;

pub struct DebugComponent {
    mouse_x: f32,
    mouse_y: f32,
    text: Text,
}

impl DebugComponent {
    pub fn new() -> Self {
        let mut text = Text::default();
        text.set_scale(PxScale::from(36.));
        text.set_font("Bold");

        Self {
            text,
            mouse_x: 0.,
            mouse_y: 0.,
        }
    }
}

impl Draw for DebugComponent {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer) {
        if ctx.time.ticks() % 10 == 0 {
            let t = format!("当前地图: {} 当前位置: {:03}|{:03} 鼠标: {:04.0}|{:04.0} FPS: {:.2}",
                            state.map.map_title, state.map.sprite_tile_x, state.map.sprite_tile_y, self.mouse_x, self.mouse_y, ctx.time.fps()
            );
            self.text.clear();
            self.text.add(t);



        }
        canvas.draw(&self.text, DrawParam::new().dest(vec2(10., 10.)).color(Color::from_rgb_u32(0x0c8918)));
    }
}

impl Event for DebugComponent {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, _state: &mut State) {
        self.mouse_x = _x;
        self.mouse_y = _y;
    }
}