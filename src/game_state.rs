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
    pub thing_graph: ThingGraph,
    pub coord_map: FxHashMap<Coord, MyIndex>,
    pub event_queue: Vec<GameEvent>,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            input_state: InputState::Main,
            player_id: MyIndex::new(0),
            thing_graph: ThingGraph::new(),
            coord_map: FxHashMap::default(),
            event_queue: Vec::new(),
        };
        gs.init_world();
        gs
    }
    fn run(&mut self) {}

    pub fn init_world(&mut self) {
        // 1. Create starting location

        let room = self
            .thing_graph
            .add_node(Thing::Location(Location::CommonRoom));
        let tavern = self
            .thing_graph
            .add_node(Thing::MetaLocation(MetaLocation::Tavern));

        // 2. Create player

        let player_node = self.thing_graph.add_node(Thing::Beast(Beast::Human));

        self.coord_map.insert(Coord { x: 1, y: 1 }, player_node);

        let rat = self.thing_graph.add_node(Thing::Beast(Beast::Animal));
        let liz = self.thing_graph.add_node(Thing::Beast(Beast::Human));

        self.player_id = player_node;

        let cup = self.thing_graph.add_node(Thing::Item(Item::Cup));
        let table = self.thing_graph.add_node(Thing::Item(Item::Table));
        let bar = self.thing_graph.add_node(Thing::Item(Item::Bar));
        let chair = self.thing_graph.add_node(Thing::Item(Item::Chair));

        self.thing_graph
            .add_edge(liz, bar, GameEdge::Relation(Relation::Behind));
        self.thing_graph
            .add_edge(cup, table, GameEdge::Relation(Relation::On));
        self.thing_graph
            .add_edge(chair, rat, GameEdge::Relation(Relation::On));
        self.thing_graph
            .add_edge(player_node, chair, GameEdge::Relation(Relation::Sitting));
        self.thing_graph
            .add_edge(chair, table, GameEdge::Relation(Relation::At));

        self.thing_graph
            .add_edge(room, tavern, GameEdge::Relation(Relation::OfMeta));

        self.thing_graph
            .add_edge(table, room, GameEdge::Relation(Relation::In));

        self.thing_graph
            .add_edge(bar, room, GameEdge::Relation(Relation::In));
    }
    pub fn describe_player_location(&self) -> String {
        let player_node = self.player_id;

        self.thing_graph.describe_location(player_node)
    }
}
