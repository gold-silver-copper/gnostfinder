use crate::*;

// Menu structure
#[derive(Resource)]
pub struct GameState {
    pub input_state: InputState,
    pub player_id: usize,
}
