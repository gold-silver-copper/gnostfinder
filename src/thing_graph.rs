use petgraph::{
    algo::{Measure, astar},
    prelude::StableDiGraph,
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

                if edge.weight() == &GameEdge::Relation(Relation::OfMeta) {
                    in_id = Some(target);
                }
            }
        }

        if let Some(in_id) = in_id {
            if let Some(edge_star) = astar_with_edges(self, thing_id, |f| f == in_id, |_| 1, |_| 0)
            {
                for (source, edge_type, target) in edge_star {
                    let source_name = self[source].name(); // use display_name() if needed

                    let target_name = self[target].name(); // use display_name() if needed

                    let phrase = edge_type.describe_to(&target_name);

                    description.push_str(&format!("{source_name} {}, ", phrase));
                }
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
        description.push_str(&format!("                        "));
        let boop = source_nodes(self);
        for x in boop {
            let source_name = self[x].name(); // use display_name() if needed
            description.push_str(&format!(" sources {source_name}  "));
        }

        description
    }
    // --- Connections ---
    fn add_door(&mut self, a: NodeIndex, b: NodeIndex) {
        self.add_edge(a, b, GameEdge::Connection(Connection::Door));
        self.add_edge(b, a, GameEdge::Connection(Connection::Door));
    }
}

pub type ThingGraph = StableDiGraph<Thing, GameEdge>;
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
    OfMeta,
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
            Relation::OfMeta => format!("of {}", target),
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

fn astar_with_edges<N, E, FN, IN, FS, K>(
    graph: &StableDiGraph<N, E>,
    start: NodeIndex,
    is_goal: FN,
    mut edge_cost: FS,
    mut estimate: IN,
) -> Option<Vec<(NodeIndex, E, NodeIndex)>>
where
    N: Clone,
    E: Clone,
    K: std::ops::Add<Output = K> + Ord + Copy + Default + Measure,
    FN: Fn(NodeIndex) -> bool,
    IN: FnMut(NodeIndex) -> K,
    FS: FnMut(&E) -> K,
{
    if let Some((_, nodes)) = astar(
        graph,
        start,
        is_goal,
        |e| edge_cost(e.weight()),
        |n| estimate(n),
    ) {
        // Convert node path to edge-annotated path
        let mut path_with_edges = Vec::new();
        for w in nodes.windows(2) {
            if let [a, b] = *w {
                if let Some(edge_ref) = graph.find_edge(a, b) {
                    let edge_weight = graph.edge_weight(edge_ref).unwrap().clone();
                    path_with_edges.push((a, edge_weight, b));
                }
            }
        }
        Some(path_with_edges)
    } else {
        None
    }
}

fn source_nodes<N, E>(graph: &StableDiGraph<N, E>) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|&n| graph.neighbors_directed(n, Direction::Incoming).count() == 0)
        .collect()
}

fn terminal_nodes<N, E>(graph: &StableDiGraph<N, E>) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|&n| {
            graph
                .neighbors_directed(n, petgraph::Direction::Outgoing)
                .count()
                == 0
        })
        .collect()
}
