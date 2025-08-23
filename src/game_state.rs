use bevy::platform::collections::{HashMap, HashSet};

use crate::*;

// Menu structure
#[derive(Resource)]
pub struct GameState {
    pub input_state: InputState,
    pub player_id: ID,
    pub location_map: LocationMap,
}

pub type LocationMap = HashMap<ID, Location>;
pub struct Location {
    contained_things: ThingSet,
}
pub type ThingSet = HashSet<Thing>;
pub struct Thing {
    id: ID,
}
pub type ID = usize;

impl GameState {
    fn run(&mut self) {
        self.input_handler();
    }

    fn input_handler(&mut self) {
        match self.input_state {
            InputState::Down => (),
            InputState::Up => (),
            InputState::Select => (),
            InputState::Back => {
                panic!("aaa");
            }
            _ => {}
        }
    }
}

fn game_state_run(mut game_state: ResMut<GameState>) {
    game_state.run();
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn game_state_plugin(app: &mut App) {
    app.add_systems(Update, game_state_run);
}
