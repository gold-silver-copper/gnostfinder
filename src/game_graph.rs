use crate::*;

pub trait GameNode {
    fn name(&self) -> String;
}
pub trait GameEdge {}

pub struct GameGraph<N: GameNode, E: GameEdge>(pub Graph<N, E>);
pub type GamesGraph = Graph<dyn GameNode, dyn GameEdge>;

#[derive(Clone, Debug)]
pub enum MetaLocation {
    Tavern,
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
