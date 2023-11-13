use ggez::{Context, GameError};
use ggez::event::MouseButton;
use ggez::glam::vec2;
use ggez::graphics::{BlendComponent, BlendFactor, BlendMode, BlendOperation, Canvas, Color, DrawParam, Image, ScreenImage};
use ggez::mint::Vector2;
use crate::component::{Controller, Draw, Event, Layer};
use crate::component::debug::DebugComponent;
use crate::component::map::MapComponent;
use crate::component::sprite::SpriteComponent;
use crate::event::AppEventHandler;
use crate::state::State;

pub struct PlayerScene {
    map_component: MapComponent,
    sprite_component: SpriteComponent,
    debug_component: DebugComponent,
    map_image: Image,
    object_image: Image,
}

impl PlayerScene {

    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        state.map.reload_map(1, 1, 1, "n3", "test", 333, 333, 0, 0);


        let map = MapComponent::new(state, 1600.0, 1200.0);
        let map_image = ScreenImage::new(ctx, None, 1.0, 1.0, 1).image(ctx);
        let object_image = ScreenImage::new(ctx, None, 1.0, 1.0, 1).image(ctx);
        Self {
            map_component: map,
            debug_component: DebugComponent::new(),
            sprite_component: SpriteComponent::new(ctx, state),
            map_image,
            object_image,
        }
    }
}

impl AppEventHandler<GameError> for PlayerScene {
    fn update(&mut self, ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        self.map_component.update(ctx, state);
        self.sprite_component.update(ctx, state);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        // self.map.draw_tile()
        let mut map_canvas = Canvas::from_image(ctx, self.map_image.clone(), Color::new(0., 0., 0., 0.));
        let mut object_canvas = Canvas::from_image(ctx, self.object_image.clone(), Color::new(0., 0., 0., 0.));
        self.map_component.draw(ctx, &mut object_canvas, state, Layer::MapTile);
        self.map_component.draw(ctx, &mut object_canvas, state, Layer::MapMiddle);


        self.sprite_component.draw(ctx, &mut object_canvas, state, Layer::MapObjects);
        self.map_component.draw(ctx, &mut object_canvas, state, Layer::MapObjects);

        map_canvas.finish(ctx)?;
        object_canvas.finish(ctx)?;



        let mut canvas = Canvas::from_frame(ctx, Color::new(0., 0., 0., 0.));
        canvas.draw(&self.map_image, DrawParam::default().scale(vec2(1.5, 1.5)));
        canvas.draw(&self.object_image, DrawParam::default().scale(vec2(1.5, 1.5)));
        // canvas.draw(&self.map_image, DrawParam::default());
        self.debug_component.draw(ctx, &mut canvas, state, Layer::Debug);
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32, state: &mut State) -> Result<(), GameError> {
        self.map_component.mouse_button_down_event(_ctx, button, x, y, state);
        self.debug_component.mouse_button_down_event(_ctx, button, x, y, state);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        self.map_component.mouse_button_up_event(_ctx, _button, _x, _y, _state);
        self.debug_component.mouse_button_up_event(_ctx, _button, _x, _y, _state);
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, _state: &mut State) -> Result<(), GameError> {
        self.map_component.mouse_motion_event(_ctx, _x, _y, _dx, _dy, _state);
        self.debug_component.mouse_motion_event(_ctx, _x, _y, _dx, _dy, _state);
        Ok(())
    }
}