use crate::*;

fn game_state_run(mut game_state: ResMut<GameState>) {
    game_state.run();
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn game_state_plugin(app: &mut App) {
    app.add_systems(Update, game_state_run);
}
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub type MyIndex = NodeIndex<u32>;
pub type Subject = MyIndex;
pub type Object = MyIndex;
// Menu structure
#[derive(Resource)]
pub struct GameState {
    pub input_state: InputState,
    pub player_id: MyIndex,

    pub event_queue: Vec<GameEvent>,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            input_state: InputState::Main,
            player_id: MyIndex::new(0),

            event_queue: Vec::new(),
        };
        gs.init_world();
        gs
    }
    fn run(&mut self) {}

    pub fn init_world(&mut self) {}
}
