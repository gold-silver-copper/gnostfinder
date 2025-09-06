use crate::*;

#[derive(Clone, Debug)]
pub enum Thing {
    MetaLocation(MetaLocation),
    Location(Location),
    Beast(Beast),
    Item(Item),
}
#[derive(Clone, Debug)]
pub enum MetaLocation {
    Tavern,
}
/// Type of location (helps categorize behavior, appearance, etc.)
#[derive(Clone, Debug)]
pub enum Location {
    CommonRoom,
    GuestRoom,
}

/// Type of item
#[derive(Clone, Debug)]
pub enum Item {
    Cup,
    Table,
    Bar,
    Chair,
}
/// Type of item
#[derive(Clone, Debug)]
pub enum Beast {
    Human,
    Animal,
}

pub trait GameNode {
    fn name(&self) -> String;
}

impl GameNode for MetaLocation {
    fn name(&self) -> String {
        format! {"{:#?}",self}
    }
}
impl GameNode for Location {
    fn name(&self) -> String {
        format! {"{:#?}",self}
    }
}
impl GameNode for Item {
    fn name(&self) -> String {
        format! {"{:#?}",self}
    }
}
impl GameNode for Beast {
    fn name(&self) -> String {
        format! {"{:#?}",self}
    }
}

impl Thing {
    /// Get the "display name" of the thing.
    pub fn name(&self) -> String {
        match self {
            Thing::Location(x) => x.name(),
            Thing::Beast(x) => x.name(),
            Thing::Item(x) => x.name(),
            Thing::MetaLocation(x) => x.name(),
        }
    }
}
