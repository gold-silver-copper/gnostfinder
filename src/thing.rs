use crate::*;

pub type ThingMap = HashMap<ID, Thing>;
#[derive(Clone)]
pub struct Thing {}

impl Thing {
    pub fn new() -> Self {
        Thing {}
    }
}
