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

        let town_node = self.thing_graph.add_node(Thing::new_location(
            "Oakvale",
            "A quiet town with cobbled streets.",
            LocationType::Town,
        ));

        // 2. Create player

        let player_node = self
            .thing_graph
            .add_node(Thing::new_person(CharacterSheet::default()));

        self.player_id = player_node;

        // 3. Create a starting item

        let sword_node = self
            .thing_graph
            .add_node(Thing::new_item("Iron Sword", ItemType::Weapon));

        // 4. Connect player to location (Inside)
        self.thing_graph
            .add_edge(player_node, town_node, GameEdge::Relation(RelationType::At));

        // 5. Place item in location (Inside)
        self.thing_graph
            .add_edge(sword_node, town_node, GameEdge::Relation(RelationType::At));

        // 6. Optional: add another location and connect via a passage

        let forest_node = self.thing_graph.add_node(Thing::new_location(
            "Whispering Woods",
            "The trees seem alive here.",
            LocationType::Forest,
        ));

        self.thing_graph.add_edge(
            town_node,
            forest_node,
            GameEdge::Passage(PassageType::Street),
        );

        // Now you have a simple world graph:
        // Town <-> Forest, Player and Sword inside Town
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
