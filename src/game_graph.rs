use crate::*;

pub trait NodeTrait {
    fn name(&self) -> String;
}
pub trait EdgeTrait {}

pub struct GameGraph<N: NodeTrait, E: EdgeTrait>(pub Graph<N, E>);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEdge {
    Relation(Relation),
    Connection(Connection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relation {
    Of,
    In,
    Sitting,
    At,
    On,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connection {
    Door,
}
