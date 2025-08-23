use petgraph::graph::NodeIndex;

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
    pub location_graph: LocationGraph,
    id_counter: ID,
}

pub type ID = usize;
pub type LocationGraph = Graph<Location, Connection>;

impl GameState {
    pub fn new() -> Self {
        GameState {
            input_state: InputState::None,
            player_id: 0,
            thing_graph: LocationGraph::new(),
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

    fn new_location(&mut self) -> NodeIndex {
        let location = Location::new();
        let location_id = self.location_graph.add_node(location);
        location_id.clone()
    }
    fn new_thing(&mut self, location_id: NodeIndex) -> ID {
        let thing_id = self.new_id();
        let thing = Thing::new();

        if let Some(location) = self.location_graph.node_weight_mut(location_id) {
            location.insert_thing(thing_id.clone(), thing);
        }

        let a = self
            .location_graph
            .get_mut(&location_id)
            .expect("TRIED TO GET A LOCATION THAT DOESNT EXIST");
        a.insert_thing(thing_id.clone(), thing);

        thing_id.clone()
    }

    fn init_world(&mut self) {
        let start = self.new_location();
        let player = self.new_thing(start);
        self.player_id = player;
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
