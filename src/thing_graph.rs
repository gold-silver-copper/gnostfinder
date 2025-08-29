use petgraph::{
    algo::astar,
    visit::{Dfs, Reversed},
};

use crate::*;

pub trait MyGraph {
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex);

    fn describe_location(&self, thing_id: NodeIndex) -> String;
}

impl MyGraph for ThingGraph {
    fn describe_location(&self, thing_id: NodeIndex) -> String {
        let mut description = String::from("You are ");
        let mut in_id = None;

        let mut dfs = Dfs::new(&self, thing_id);
        while let Some(nx) = dfs.next(&self) {
            let edges = self.edges(nx);
            for edge in edges {
                let target = edge.target();
                let target_name = self[target].name(); // use display_name() if needed
                let edge_type = edge.weight();
                let phrase = edge_type.describe_to(&target_name);
                if edge_type == &GameEdge::Relation(Relation::In) {
                    in_id = Some(target);
                }

                description.push_str(&format!("{}, ", phrase));
            }
        }
        description.push_str(&format!("                        "));

        if let Some(in_id) = in_id {
            let reversed = Reversed(self);
            let mut dfs = Dfs::new(&reversed, in_id);
            while let Some(nx) = dfs.next(&reversed) {
                let edges = self.edges(nx);
                for edge in edges {
                    let source = edge.source();
                    let source_name = self[source].name(); // use display_name() if needed
                    let target = edge.target();
                    let target_name = self[target].name(); // use display_name() if needed
                    let edge_type = edge.weight();
                    let phrase = edge_type.describe_to(&target_name);

                    description.push_str(&format!("{source_name} {}, ", phrase));
                }
            }
        }

        description
    }
    // --- Connections ---
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex) {
        self.add_edge(a, b, GameEdge::Connection(Connection::Door));
        self.add_edge(b, a, GameEdge::Connection(Connection::Door));
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
    Of,
    In,
    Sitting,
    At,
    On,
    Behind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connection {
    Door,
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
            Relation::Of => format!("of {}", target),
            Relation::In => format!("in {}", target),
            Relation::Sitting => format!("sitting {}", target),
            Relation::At => format!("at {}", target),
            Relation::On => format!("on {}", target),
            Relation::Behind => format!("behind {}", target),
        }
    }
}

impl Connection {
    fn describe_to(&self, target: &str) -> String {
        match self {
            Connection::Door => format!("door to {}", target),
        }
    }
}
