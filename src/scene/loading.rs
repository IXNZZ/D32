use ggez::{Context, GameError};
use crate::event::AppEventHandler;
use crate::state::State;

pub struct LoadingScene {

}

impl AppEventHandler<GameError> for LoadingScene {
    fn update(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        Ok(())
    }
}