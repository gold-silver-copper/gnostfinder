use crate::*;

pub trait MyGraph {
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex);
    fn add_passageway(&mut self, a: NodeIndex, b: NodeIndex);
    fn add_window(&mut self, a: NodeIndex, b: NodeIndex);

    fn add_north_south(&mut self, north: NodeIndex, south: NodeIndex);
    fn add_east_west(&mut self, east: NodeIndex, west: NodeIndex);

    fn add_bidirectional_relation(&mut self, a: NodeIndex, b: NodeIndex, relation: Relation);
}

impl MyGraph for ThingGraph {
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
