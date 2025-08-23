use crate::*;

pub type Stat = isize;
#[derive(Clone, Debug)]
pub struct CharacterSheet {
    pub name: String,
    life_path: LifePath,

    stats: Stats,
}

impl Default for CharacterSheet {
    fn default() -> Self {
        CharacterSheet {
            name: "Tol".into(),
            life_path: LifePath::default(),
            stats: Stats::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct LifePath {
    occupation: Occupation,
    nationality: Nationality,
}
impl Default for LifePath {
    fn default() -> Self {
        LifePath {
            occupation: Occupation::Soldier,
            nationality: Nationality::Ukrainian,
        }
    }
}
impl Default for Stats {
    fn default() -> Self {
        Stats {
            strength: 1,
            agility: 1,
            intelligence: 1,
            charisma: 1,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Stats {
    strength: Stat,
    agility: Stat,
    intelligence: Stat,
    charisma: Stat,
}
#[derive(Clone, Debug)]
pub enum Occupation {
    Soldier,
    Hunter,
}
#[derive(Clone, Debug)]
pub enum Nationality {
    Polish,
    Russian,
    Ukrainian,
    German,
}
