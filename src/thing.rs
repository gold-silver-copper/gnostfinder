use crate::*;

#[derive(Clone, Debug)]
pub enum Thing {
    Location(Location),
    Person(CharacterSheet),
    Item(Item),
}

/// A location in the world graph.
#[derive(Clone, Debug)]
pub struct Location {
    pub name: String,

    pub kind: LocationType,
}

/// Type of location (helps categorize behavior, appearance, etc.)
#[derive(Clone, Debug)]
pub enum LocationType {
    Room,
    Hall,
    Street,
}

/// An item in the world.
#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub kind: ItemType,
}

/// Type of item
#[derive(Clone, Debug)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
    Key,
}

impl Thing {
    /// Create a new location.
    pub fn new_location(name: &str, kind: LocationType) -> Self {
        Thing::Location(Location {
            name: name.to_string(),

            kind,
        })
    }

    /// Create a new person.
    pub fn new_person(cs: CharacterSheet) -> Self {
        Thing::Person(cs)
    }

    /// Create a new item.
    pub fn new_item(name: &str, kind: ItemType) -> Self {
        Thing::Item(Item {
            name: name.to_string(),
            kind,
        })
    }

    /// Get the "display name" of the thing.
    pub fn name(&self) -> &str {
        match self {
            Thing::Location(loc) => &loc.name,
            Thing::Person(person) => &person.name,
            Thing::Item(item) => &item.name,
        }
    }
}
