use ggez::{Context, GameError};
use ggez::graphics::{Canvas, Color, DrawParam, Image, ScreenImage};
use crate::component::{Controller, Draw, Layer};
use crate::component::map::MapComponent;
use crate::event::AppEventHandler;
use crate::state::State;

pub struct PlayerScene {
    map_component: MapComponent,
    map_image: Image,
    object_image: Image,
}

impl PlayerScene {

    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        state.map.reload_map(1, 1, 1, "n3", "test", 333, 333, 0, 0);


        let map = MapComponent::new(state, state.window_width, state.window_height);
        let map_image = ScreenImage::new(ctx, None, 1.0, 1.0, 1).image(ctx);
        let object_image = ScreenImage::new(ctx, None, 1.0, 1.0, 1).image(ctx);
        Self {
            map_component: map,
            map_image,
            object_image,
        }
    }
}

impl AppEventHandler<GameError> for PlayerScene {
    fn update(&mut self, ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        self.map_component.update(ctx, state);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        // self.map.draw_tile()
        let mut map_canvas = Canvas::from_image(ctx, self.map_image.clone(), Color::new(0., 0., 0., 0.));
        let mut object_canvas = Canvas::from_image(ctx, self.object_image.clone(), Color::new(0., 0., 0., 0.));
        self.map_component.draw(ctx, &mut map_canvas, _state, Layer::MapTile);
        self.map_component.draw(ctx, &mut map_canvas, _state, Layer::MapMiddle);
        self.map_component.draw(ctx, &mut object_canvas, _state, Layer::MapObjects);

        map_canvas.finish(ctx)?;
        object_canvas.finish(ctx)?;
        let mut canvas = Canvas::from_frame(ctx, Color::new(0., 0., 0., 0.));
        canvas.draw(&self.map_image, DrawParam::default());
        canvas.draw(&self.object_image, DrawParam::default());
        // canvas.draw(&self.map_image, DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}