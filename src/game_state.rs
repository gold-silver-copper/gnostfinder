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
    pub last_input: LastInput,
    pub input_state: InputState,
    pub player_id: NodeIndex,
    pub thing_graph: ThingGraph,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            last_input: LastInput::None,
            input_state: InputState::Main,
            player_id: NodeIndex::new(0),
            thing_graph: ThingGraph::new(),
        };
        gs.init_world();
        gs
    }
    fn run(&mut self) {
        self.input_handler();
    }

    pub fn init_world(&mut self) {
        // 1. Create starting location

        let room1 = self
            .thing_graph
            .add_node(Thing::new_location("Room1", LocationType::Room));
        let room2 = self
            .thing_graph
            .add_node(Thing::new_location("Room2", LocationType::Room));
        let hall = self
            .thing_graph
            .add_node(Thing::new_location("Hall", LocationType::Hall));
        let lobby = self
            .thing_graph
            .add_node(Thing::new_location("Lobby", LocationType::Hall));
        let street = self
            .thing_graph
            .add_node(Thing::new_location("street", LocationType::StreetSection));

        // 2. Create player

        let player_node = self
            .thing_graph
            .add_node(Thing::new_person(CharacterSheet::default()));

        self.player_id = player_node;

        self.thing_graph.contained_by(player_node, room1);

        // 6. Optional: add another location and connect via a passage

        self.thing_graph.add_north_south(room2, room1);
        self.thing_graph.add_open_door(room1, hall);
        self.thing_graph.add_open_door(room2, hall);

        self.thing_graph.add_passageway(hall, lobby);
        self.thing_graph.add_open_door(lobby, street);

        // Now you have a simple world graph:
    }
    pub fn describe_player_location(&self) -> String {
        let player_node = self.player_id;

        self.thing_graph.describe_location(player_node)
    }
}
