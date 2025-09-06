use petgraph::graph::NodeIndex;
use rustc_hash::FxHashMap;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut map: FxHashMap<Coord, NodeIndex> = FxHashMap::default();
    let player: NodeIndex<u32> = NodeIndex::new(3);

    map.insert(Coord { x: 5, y: -10 }, player);
}
