use bevy::platform::collections::{HashMap, HashSet};

use crate::*;

fn game_state_run(mut game_state: ResMut<GameState>) {
    game_state.run();
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn game_state_plugin(app: &mut App) {
    app.add_systems(Update, game_state_run);
}

// Menu structure
#[derive(Resource)]
pub struct GameState {
    pub input_state: InputState,
    pub player_id: ID,
    pub location_map: LocationMap,
    id_counter: ID,
}

pub type LocationMap = HashMap<ID, Location>;
pub struct Location {
    contained_things: ThingSet,
    connections_to: HashSet<Connection>,
}
pub struct Connection {
    to_location: ID,
    connection_type: ConnectionType,
}
pub enum ConnectionType {
    Door,
}

pub type ThingSet = HashSet<Thing>;
pub struct Thing {
    id: ID,
}
pub type ID = usize;

impl GameState {
    pub fn new() -> Self {
        GameState {
            input_state: InputState::None,
            player_id: 0,
            location_map: LocationMap::new(),
            id_counter: 0,
        }
    }
    fn run(&mut self) {
        self.input_handler();
    }

    fn create_location(&mut self) {}

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
