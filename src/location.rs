use crate::*;

pub type LocationMap = HashMap<ID, Location>;
#[derive(Clone)]
pub struct Location {
    contained_things: ThingMap,
    connections_to: ConnectionSet,
}
#[derive(Clone)]
pub struct Connection {
    to_location: ID,
    connection_type: ConnectionType,
}

pub type ConnectionSet = HashSet<Connection>;
#[derive(Clone)]
pub enum ConnectionType {
    Door,
}

impl Location {
    pub fn new() -> Location {
        Location {
            contained_things: ThingMap::new(),
            connections_to: ConnectionSet::new(),
        }
    }
}
