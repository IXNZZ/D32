use ggez::Context;
use ggez::graphics::Canvas;
use crate::cache::ImageCache;
use crate::control::GameState;
use crate::easing::{Easing, Point2};

pub struct MapControl {
    move_x: f32,
    move_y: f32,
    map_title: String,
    walk_easing: Easing<Point2>,
    run_easing: Easing<Point2>,
}

impl MapControl {

    // pub fn new(state: &GameState) -> Self {
    //     Self {
    //         move_x: 0.,
    //         move_y: 0.,
    //         map_title: String::from(""),
    //         // draw: MapDraw::new(state.base_dir.as_path(), 1, 1, "n0", state.window_size.0, state.window_size.1),
    //         walk_easing: Easing::new(Point2 {x: 0., y: 0.}, Point2 {x: 48., y: 32.}, 1.0),
    //         run_easing: Easing::new(Point2 {x: 0., y: 0.}, Point2 {x: 96., y: 64.}, 1.0),
    //     }
    // }
    //
    // pub fn update() {
    //
    // }
    //
    // pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, state: &mut GameState, cache: &mut ImageCache) {
    //     self.draw.draw_tile(canvas, ctx, cache, 255);
    // }
    //
    // pub fn select_map(&mut self, state: &GameState) {
    //     self.draw.reload_map(2, 3, "n3", 333, 333, 0, 0);
    // }
    //
    // pub fn jump_map() {
    //
    // }
    //
    // pub fn move_map() {
    //
    // }
}