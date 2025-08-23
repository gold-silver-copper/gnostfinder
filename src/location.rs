use crate::*;

pub type LocationMap = HashMap<ID, Location>;
#[derive(Clone)]
pub struct Location {
    contained_things: ThingMap,
}

#[derive(Clone)]
pub enum Connection {
    Door,
}

impl Location {
    pub fn new() -> Location {
        Location {
            contained_things: ThingMap::new(),
        }
    }
    pub fn insert_thing(&mut self, thing_id: ID, thing: Thing) {
        self.contained_things.insert(thing_id, thing);
    }
}
