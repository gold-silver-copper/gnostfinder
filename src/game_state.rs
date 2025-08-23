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
    fn new_id(&mut self) -> ID {
        self.id_counter += 1;
        self.id_counter.clone()
    }

    fn new_location(&mut self) -> ID {
        let id = self.new_id();
        let location = Location::new();
        self.location_map.insert(id.clone(), location.clone());
        id.clone()
    }
    fn new_thing(&mut self) -> ID {
        let id = self.new_id();
        let thing = Thing::new();

        id.clone()
    }

    fn init_world(&mut self) {
        let start = self.new_location();
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
