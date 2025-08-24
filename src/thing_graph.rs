use crate::*;

pub trait MyGraph {
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex);
    fn add_passageway(&mut self, a: NodeIndex, b: NodeIndex);
    fn add_window(&mut self, a: NodeIndex, b: NodeIndex);

    fn add_north_south(&mut self, north: NodeIndex, south: NodeIndex);
    fn add_east_west(&mut self, east: NodeIndex, west: NodeIndex);

    fn add_bidirectional_relation(&mut self, a: NodeIndex, b: NodeIndex, relation: Relation);
    fn describe_location(&self, thing_id: NodeIndex) -> String;
}

impl MyGraph for ThingGraph {
    fn describe_location(&self, thing_id: NodeIndex) -> String {
        let mut description = String::new();

        // 1. Find where the player is (edge: player --Contains--> location)
        let player_node = thing_id;

        let location = self
            .edges(player_node)
            .find(|edge| *edge.weight() == GameEdge::Relation(Relation::Contains))
            .map(|edge| edge.target());

        if let Some(loc) = location {
            let loc_name = self[loc].name();
            description.push_str(&format!("You are in {}.\n", loc_name));

            // 2. Look at edges FROM the location to other locations
            for edge in self.edges(loc) {
                let target = edge.target();
                let edge_type = edge.weight();
                let target_name = self[target].name(); // use display_name() if needed

                let phrase = edge_type.describe_to(target_name);

                description.push_str(&format!("{}\n", phrase));
            }
        } else {
            description.push_str("You are nowhere!\n");
        }

        description
    }
    // --- Connections ---
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex) {
        self.add_edge(a, b, GameEdge::Connection(Connection::Door));
        self.add_edge(b, a, GameEdge::Connection(Connection::Door));
    }

    fn add_passageway(&mut self, a: NodeIndex, b: NodeIndex) {
        self.add_edge(a, b, GameEdge::Connection(Connection::Passageway));
        self.add_edge(b, a, GameEdge::Connection(Connection::Passageway));
    }

    fn add_window(&mut self, a: NodeIndex, b: NodeIndex) {
        self.add_edge(a, b, GameEdge::Connection(Connection::Window));
        self.add_edge(b, a, GameEdge::Connection(Connection::Window));
    }

    // --- Cardinal directions ---
    fn add_north_south(&mut self, north: NodeIndex, south: NodeIndex) {
        self.add_edge(north, south, GameEdge::Relation(Relation::SouthOf));
        self.add_edge(south, north, GameEdge::Relation(Relation::NorthOf));
    }

    fn add_east_west(&mut self, east: NodeIndex, west: NodeIndex) {
        self.add_edge(east, west, GameEdge::Relation(Relation::WestOf));
        self.add_edge(west, east, GameEdge::Relation(Relation::EastOf));
    }

    // --- Generic bidirectional for other relations ---
    fn add_bidirectional_relation(&mut self, a: NodeIndex, b: NodeIndex, relation: Relation) {
        use Relation::*;
        match relation {
            // Automatically add the inverse for cardinal directions
            NorthOf => self.add_north_south(a, b),
            SouthOf => self.add_north_south(b, a),
            EastOf => self.add_east_west(a, b),
            WestOf => self.add_east_west(b, a),

            // Symmetric relations: just add in both directions
            OnTopOf | Underneath | AttachedTo | NextTo => {
                self.add_edge(a, b, GameEdge::Relation(relation));
                self.add_edge(b, a, GameEdge::Relation(relation));
            }

            // Asymmetric, don’t auto-invert
            Contains => {
                self.add_edge(a, b, GameEdge::Relation(Contains));
            }

            Above | Below => {
                self.add_edge(a, b, GameEdge::Relation(relation));
                // optionally invert if desired:
                // self.add_edge(b, a, GameEdge::Relation(match relation {
                //     Above => Below,
                //     Below => Above,
                //     _ => relation,
                // }));
            }
        }
    }
}

pub type ThingGraph = Graph<Thing, GameEdge>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEdge {
    Relation(Relation),
    Connection(Connection),
}

/// Edge types for spatial relations between entities in the game world.
///
/// Combines:
/// - RCC-8 style topology (containment, touching, overlap).
/// - Cardinal directions (north, south, etc.).
/// - Relative orientation (left/right, above/below).
/// - Functional/contact (on top of, attached to, next to).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relation {
    // --- Topological (RCC-like) ---
    Contains, // A contains B

    // --- Cardinal / Directional ---
    NorthOf,
    SouthOf,
    EastOf,
    WestOf,

    // --- Relative / Orientation ---
    Above,
    Below,

    // --- Functional / Contact ---
    OnTopOf,    // A is physically supported by B
    Underneath, // A is beneath and supported by B
    AttachedTo, // A is fastened to B (painting on wall)
    NextTo,     // A is adjacent to B without overlap
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connection {
    Door,
    Passageway,
    Window,
}

impl GameEdge {
    /// Returns a human-readable description of the edge to a target.
    pub fn describe_to(&self, target: &str) -> String {
        match self {
            GameEdge::Relation(r) => r.describe_to(target),
            GameEdge::Connection(c) => c.describe_to(target),
        }
    }
}

impl Relation {
    fn describe_to(&self, target: &str) -> String {
        match self {
            Relation::NorthOf => format!("to the north is {}", target),
            Relation::SouthOf => format!("to the south is {}", target),
            Relation::EastOf => format!("to the east is {}", target),
            Relation::WestOf => format!("to the west is {}", target),
            Relation::Contains => format!("contains {}", target),
            Relation::OnTopOf => format!("on top of {}", target),
            Relation::NextTo => format!("next to {}", target),
            Relation::Above => format!("above {}", target),
            Relation::Below => format!("below {}", target),
            Relation::Underneath => format!("underneath {}", target),
            Relation::AttachedTo => format!("attached to {}", target),
        }
    }
}

impl Connection {
    fn describe_to(&self, target: &str) -> String {
        match self {
            Connection::Door => format!("there is a door to {}", target),
            Connection::Passageway => format!("continues to {}", target),
            Connection::Window => format!("there is a window to {}", target),
        }
    }
}
