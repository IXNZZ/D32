use ggez::{Context, GameError};
use crate::component::map::MapComponent;
use crate::event::AppEventHandler;
use crate::state::State;

pub struct PlayerScene {
    map: MapComponent
}

impl PlayerScene {

    pub fn new(ctx: &mut Context, state: &mut State) -> Self {
        let map = MapComponent::new(&state.base_dir,
                                    state.map.map_data_id,
                                    state.map.map_data_number, "n3", "test", state.window_width, state.window_height);
        Self {
            map
        }
    }
}

impl AppEventHandler<GameError> for PlayerScene {
    fn update(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        // self.map.draw_tile()

        Ok(())
    }
}