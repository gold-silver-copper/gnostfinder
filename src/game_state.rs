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
            .add_node(Thing::new_location("street", LocationType::Street));

        // 2. Create player

        let player_node = self
            .thing_graph
            .add_node(Thing::new_person(CharacterSheet::default()));

        self.player_id = player_node;

        // 4. Connect player to location (Inside)
        self.thing_graph
            .add_edge(player_node, room1, GameEdge::Relation(Relation::Contains));

        // 6. Optional: add another location and connect via a passage

        self.thing_graph.add_north_south(room2, room1);
        self.thing_graph.add_door(room1, hall);
        self.thing_graph.add_door(room2, hall);

        self.thing_graph.add_passageway(hall, lobby);
        self.thing_graph.add_door(lobby, street);

        // Now you have a simple world graph:
    }
    pub fn describe_location(&self) -> String {
        let mut description = String::new();

        // 1. Find where the player is (edge: player --Contains--> location)
        let player_node = self.player_id;

        let location = self
            .thing_graph
            .edges(player_node)
            .find(|edge| *edge.weight() == GameEdge::Relation(Relation::Contains))
            .map(|edge| edge.target());

        if let Some(loc) = location {
            let loc_name = self.thing_graph[loc].name();
            description.push_str(&format!("You are in {}.\n", loc_name));

            // 2. Look at edges FROM the location to other locations
            for edge in self.thing_graph.edges(loc) {
                let target = edge.target();
                let edge_type = edge.weight();
                let target_name = self.thing_graph[target].name(); // use display_name() if needed

                let phrase = match edge_type {
                    GameEdge::Relation(r) => match r {
                        Relation::NorthOf => format!("to the north is {}", target_name),
                        Relation::SouthOf => format!("to the south is {}", target_name),
                        Relation::EastOf => format!("to the east is {}", target_name),
                        Relation::WestOf => format!("to the west is {}", target_name),
                        Relation::Contains => format!("contains {}", target_name),
                        Relation::OnTopOf => format!("on top of {}", target_name),
                        Relation::NextTo => format!("next to {}", target_name),
                        _ => format!("related to {}", target_name),
                    },
                    GameEdge::Connection(c) => match c {
                        Connection::Door => format!("there is a door to {}", target_name),
                        Connection::Passageway => format!("continues to {}", target_name),
                        Connection::Window => format!("there is a window to {}", target_name),
                    },
                };

                description.push_str(&format!("{}\n", phrase));
            }
        } else {
            description.push_str("You are nowhere!\n");
        }

        description
    }
}
