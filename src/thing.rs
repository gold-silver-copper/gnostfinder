use crate::*;

#[derive(Clone, Debug)]
pub enum Thing {
    MetaLocation { name: String, meta: MetaLocation },
    Location { name: String, typik: LocationType },
    Person(CharacterSheet),
    Item { name: String, typik: ItemType },
}
#[derive(Clone, Debug)]
pub enum MetaLocation {
    House,
    City,
    Street,
}

/// Type of location (helps categorize behavior, appearance, etc.)
#[derive(Clone, Debug)]
pub enum LocationType {
    Room,
    Hall,
    StreetSection,
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
    /// Create a new meta-location.
    pub fn new_meta_location(name: &str, meta: MetaLocation) -> Self {
        Thing::MetaLocation {
            name: name.to_string(),
            meta,
        }
    }

    /// Create a new location.
    pub fn new_location(name: &str, typik: LocationType) -> Self {
        Thing::Location {
            name: name.to_string(),
            typik,
        }
    }

    /// Create a new person.
    pub fn new_person(cs: CharacterSheet) -> Self {
        Thing::Person(cs)
    }

    /// Create a new item.
    pub fn new_item(name: &str, kind: ItemType) -> Self {
        Thing::Item {
            name: name.to_string(),
            typik: kind,
        }
    }

    /// Get the "display name" of the thing.
    pub fn name(&self) -> &str {
        match self {
            Thing::Location { name, .. } => name.as_str(),
            Thing::Person(person) => &person.name,
            Thing::Item { name, .. } => name.as_str(),
            Thing::MetaLocation { name, .. } => name.as_str(),
        }
    }
}
