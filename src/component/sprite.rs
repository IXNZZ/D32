use ggez::Context;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, Drawable, DrawMode, DrawParam, Mesh, Rect, StrokeOptions, Text};
use crate::cache::CacheKey;
use crate::component::{Controller, Draw, Layer};
use crate::state::sprite::SpriteState;
use crate::state::State;

pub struct SpriteComponent{

}

impl SpriteComponent {

    pub fn new(_ctx: &mut Context, state: &mut State) -> Self {
        // test code
        state.sprite = SpriteState::new_test(state);

        // end code
        Self {}
    }

    fn draw_image(&self, canvas: &mut Canvas, state: &mut State, layer: i32, key: CacheKey, rel_x: f32, rel_y: f32) {
        if let Some(image) = state.cache.get(&key.get_data_key()) {
            let image_width = image.image().width() as f32;
            let image_height = image.image().height() as f32;
            if let Some(meta) = image.meta(key.get_meta_key()) {
                let param = DrawParam::default().src(Rect::new(meta.src_x / image_width, meta.src_y / image_height, meta.width as f32 / image_width, meta.height as f32 / image_height))
                    .dest(vec2(rel_x + meta.offset_x, rel_y + meta.offset_y));
                // println!("draw sprite");
                image.image().draw(canvas, param);
            }
        }
    }

}

impl Controller for SpriteComponent {
    fn update(&mut self, ctx: &mut Context, state: &mut State) {
        state.sprite.easing(ctx.time.delta().as_secs_f64());
    }
}

impl Draw for SpriteComponent {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut State, layer: Layer) {
        // let sprite = &state.sprite;
        let rel_x = state.sprite.abs_point_x - state.map.sprite_abs_x + state.center_x;
        let rel_y = state.sprite.abs_point_y - state.map.sprite_abs_y + state.center_y;
        let key = state.sprite.dress();
        // println!("sprite draw: {:?}", key);
        self.draw_image(canvas, state, 300, key, rel_x, rel_y);
        let key = state.sprite.hair();
        self.draw_image(canvas, state, 300, key, rel_x, rel_y);
        // let key = state.sprite.effect();
        // self.draw_image(canvas, state, 300, key, rel_x, rel_y);
        // let key = state.sprite.weapon();
        // self.draw_image(canvas, state, 300, key, rel_x, rel_y);
        // let key = state.sprite.weapon_effect();
        // self.draw_image(canvas, state, 300, key, rel_x, rel_y);

        // if let Some(image) = state.cache.get(&key.get_data_key()) {
        //     let image_width = image.image().width() as f32;
        //     let image_height = image.image().height() as f32;
        //     image.image().draw(canvas, DrawParam::new().scale(vec2(0.8, 0.9)));
        //     let len = image.all().len();
        //     // println!("meta: {}", len);
        //     for i in 0..100 {
        //         if let Some(meta) = image.meta(key.as_inc_index(i).get_meta_key()) {
        //             // let mesh = Mesh::new_rectangle(ctx,
        //             //                                DrawMode::Stroke(StrokeOptions::default()),
        //             //                                Rect::new(0.0, 0.0, meta.width as f32, meta.height as f32),
        //             //                                Color::RED).unwrap();
        //             // mesh.draw(canvas, DrawParam::new().dest(vec2(meta.src_x, meta.src_y)));
        //             let text = Text::new(format!("src:{}|{}\ni:{}|{}", meta.src_x, meta.src_y, meta.key.get_file_index(), i));
        //             text.draw(canvas, DrawParam::new().dest(vec2(meta.src_x, meta.src_y)).color(Color::GREEN));
        //         }
        //
        //     }
        // }
    }
}