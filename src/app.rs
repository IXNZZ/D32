use ggez::{Context, GameError};
use ggez::event::{ErrorOrigin, MouseButton};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::Ime;
use crate::event::AppEventHandler;
use crate::scene::MainScene;
use crate::state::State;

pub struct App {
    scene: MainScene,
}

impl App {
    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        let scene = MainScene::new(ctx, state);
        Self {
            scene,
        }
    }
}

impl AppEventHandler<GameError> for App {
    fn update(&mut self, _ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.update(_ctx, state);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        state.cache.insert_key(_ctx);
        if state.initialled {
            return self.scene.draw(_ctx, state);
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.mouse_button_down_event(_ctx, _button, _x, _y, state);
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.mouse_button_up_event(_ctx, _button, _x, _y, state);
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.mouse_motion_event(_ctx, _x, _y, _dx, _dy, state);
        }
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.mouse_enter_or_leave(_ctx, _entered, state);
        }
        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.mouse_wheel_event(_ctx, _x, _y, state);
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.key_down_event(ctx, input, _repeated, state);
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.key_up_event(_ctx, _input, state);
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.text_input_event(_ctx, _character, state);
        }
        Ok(())
    }

    fn ime_input_event(&mut self, _ctx: &mut Context, ime: Ime, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.ime_input_event(_ctx, ime, state);
        }
        Ok(())
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.focus_event(_ctx, _gained, state);
        }
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context, state: &mut State) -> Result<bool, GameError> {
        if state.initialled {
            return self.scene.quit_event(_ctx, state);
        }
        Ok(false)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32, state: &mut State) -> Result<(), GameError> {
        if state.initialled {
            return self.scene.resize_event(_ctx, _width, _height, state);
        }
        Ok(())
    }

    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: GameError, state: &mut State) -> bool {
        if state.initialled {
            return self.scene.on_error(_ctx, _origin, _e, state);
        }
        true
    }
}