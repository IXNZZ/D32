mod loading;
mod player;

use ggez::{Context, GameError};
use ggez::event::{ErrorOrigin, MouseButton};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::Ime;
use tracing::debug;
use crate::event::{AppEventHandler};
use crate::scene::loading::LoadingScene;
use crate::scene::player::PlayerScene;
use crate::state::State;

pub enum SceneState {
    Loading,
    Login,
    Role,
    Player,
}



pub struct MainScene {
    scene: Box<dyn AppEventHandler>,
}

impl MainScene {
    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        state.initialled = true;
        MainScene {scene: Box::new(PlayerScene::new(ctx, state))}
    }

    pub fn switch_scene(&mut self, _ctx: &mut Context, _state: &mut State, scene: Box<dyn AppEventHandler>) {
        self.scene = scene;
    }
}

impl AppEventHandler<GameError> for MainScene {
    fn update(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        if let Some(x) = self.scene.select(_ctx, _state) {
            self.switch_scene(_ctx, _state, x);
        }
        self.scene.update(_ctx, _state)
    }

    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        self.scene.draw(_ctx, _state)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        self.scene.mouse_button_down_event(_ctx, _button, _x, _y, _state)
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        self.scene.mouse_button_up_event(_ctx, _button, _x, _y, _state)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32, _state: &mut State) -> Result<(), GameError> {
        self.scene.mouse_motion_event(_ctx,  _x, _y, _dx, _dy, _state)
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool, _state: &mut State) -> Result<(), GameError> {
        self.scene.mouse_enter_or_leave(_ctx, _entered, _state)
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _state: &mut State) -> Result<(), GameError> {
        self.scene.mouse_wheel_event(_ctx, _x, _y, _state)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, _state: &mut State) -> Result<(), GameError> {
        self.scene.key_down_event(ctx, input, _repeated, _state)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput, _state: &mut State) -> Result<(), GameError> {
        self.scene.key_up_event(_ctx, _input, _state)
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char, _state: &mut State) -> Result<(), GameError> {
        self.scene.text_input_event(_ctx, _character, _state)
    }

    fn ime_input_event(&mut self, _ctx: &mut Context, ime: Ime, _state: &mut State) -> Result<(), GameError> {
        self.scene.ime_input_event(_ctx, ime, _state)
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool, _state: &mut State) -> Result<(), GameError> {
        self.scene.focus_event(_ctx, _gained, _state)
    }

    fn quit_event(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<bool, GameError> {
        self.scene.quit_event(_ctx, _state)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32, _state: &mut State) -> Result<(), GameError> {
        self.scene.resize_event(_ctx, _width, _height, _state)
    }

    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: GameError, _state: &mut State) -> bool {
        self.scene.on_error(_ctx, _origin, _e, _state)
    }
}

