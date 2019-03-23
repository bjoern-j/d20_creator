use std::collections::HashMap;
use crate::character::{ Ability, AbilityScore, Size, Speed, Skill };

pub struct Race {
    pub name : String,
    pub long_text : String,
    pub ability_bonuses : HashMap<Ability, AbilityScore>,
    pub size : Size,
    pub speed : Speed,
    pub languages: Vec<String>,
    pub skill_proficiencies : Vec<Skill>,
}

pub struct Datastore {
    races : HashMap<String, Race>,
}

impl Datastore {
    /// Creates a new data store without any data in it
    pub fn new() -> Self { 
        Datastore {
            races : HashMap::new(),
        }
    }
    pub fn add_race(&mut self, race : Race) {
        self.races.insert(race.name.clone(), race);
    }
    pub fn get_race(&self, race : &str) -> Option<&Race> {
        self.races.get(race)
    }
}