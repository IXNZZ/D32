use ggez::{Context, GameError};
use ggez::event::{ErrorOrigin, MouseButton};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::Ime;
use crate::event::AppEventHandler;
use crate::scene::MainScene;
use crate::state::State;

pub struct App {
    scene: MainScene,
    initialled: bool,
}

impl App {
    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        let scene = MainScene::new(ctx, state);
        Self {
            initialled: false,
            scene,
        }
    }
}

impl AppEventHandler<GameError> for App {
    fn update(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.update(_ctx, _state);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        _state.cache.insert_key(_ctx);
        if self.initialled {
            return self.scene.draw(_ctx, _state);
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.mouse_button_down_event(_ctx, _button, _x, _y, _state);
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.mouse_button_up_event(_ctx, _button, _x, _y, _state);
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.mouse_motion_event(_ctx, _x, _y, _dx, _dy, _state);
        }
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.mouse_enter_or_leave(_ctx, _entered, _state);
        }
        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.mouse_wheel_event(_ctx, _x, _y, _state);
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.key_down_event(ctx, input, _repeated, _state);
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.key_up_event(_ctx, _input, _state);
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.text_input_event(_ctx, _character, _state);
        }
        Ok(())
    }

    fn ime_input_event(&mut self, _ctx: &mut Context, ime: Ime, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.ime_input_event(_ctx, ime, _state);
        }
        Ok(())
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.focus_event(_ctx, _gained, _state);
        }
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<bool, GameError> {
        if self.initialled {
            return self.scene.quit_event(_ctx, _state);
        }
        Ok(false)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32, _state: &mut State) -> Result<(), GameError> {
        if self.initialled {
            return self.scene.resize_event(_ctx, _width, _height, _state);
        }
        Ok(())
    }

    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: GameError, _state: &mut State) -> bool {
        if self.initialled {
            return self.scene.on_error(_ctx, _origin, _e, _state);
        }
        true
    }
}