use petgraph::Direction;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

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
    pub player_id: NodeIndex,
    pub thing_graph: ThingGraph,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            input_state: InputState::None,
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

        self.thing_graph
            .add_edge(room1, room2, GameEdge::Relation(Relation::NorthOf));
        self.thing_graph
            .add_edge(room1, hall, GameEdge::Connection(Connection::Door));
        self.thing_graph
            .add_edge(room2, hall, GameEdge::Connection(Connection::Door));
        self.thing_graph
            .add_edge(lobby, hall, GameEdge::Connection(Connection::Passageway));
        self.thing_graph
            .add_edge(lobby, street, GameEdge::Connection(Connection::Door));

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
                let target_name = &self.thing_graph[target].name();

                let relation = match edge_type {
                    GameEdge::Relation(r) => match r {
                        Relation::NorthOf => "to the north",
                        Relation::SouthOf => "to the south",
                        Relation::EastOf => "to the east",
                        Relation::WestOf => "to the west",
                        Relation::Contains => "contains",
                        Relation::OnTopOf => "on top of",
                        Relation::NextTo => "next to",
                        _ => todo!(),
                    },
                    GameEdge::Connection(c) => match c {
                        Connection::Door => "has door to",
                        Connection::Passageway => "continues to",
                        Connection::Window => "has a window to",
                        // …whatever else you define
                    },
                };

                description.push_str(&format!("{} is {}.\n", target_name, relation));
            }
        } else {
            description.push_str("You are nowhere!\n");
        }

        description
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
