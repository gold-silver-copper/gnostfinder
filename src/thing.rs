use crate::*;

#[derive(Clone, Debug)]
pub enum Thing {
    MetaLocation(MetaLocation),
    Location(Location),
    Beast(Beast),
    Item(Item),
    Wall,
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

impl Thing {
    /// Get the "display name" of the thing.
    pub fn name(&self) -> String {
        format! {"{:#?}",self}
    }
}
