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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponCategory { Simple, Martial }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponRange{ Melee, Ranged }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ArmorCategory { Light, Medium, Heavy, Shield }
pub type Reach = u16;

pub struct Weapon {
    pub name : String,
    pub category : WeaponCategory,
    pub range_category : WeaponRange,
    pub reach : Reach,
}

pub struct Armor {
    pub name : String,
    pub category : ArmorCategory,
}

pub struct Datastore {
    races : HashMap<String, Race>,
    weapons : HashMap<String, Weapon>,
    armors : HashMap<String, Armor>,
}

impl Datastore {
    /// Creates a new data store without any data in it
    pub fn new() -> Self { 
        Datastore {
            races : HashMap::new(),
            weapons : HashMap::new(),
            armors : HashMap::new(),
        }
    }
    pub fn add_race(&mut self, race : Race) {
        self.races.insert(race.name.clone(), race);
    }
    pub fn add_weapon(&mut self, weapon : Weapon) {
        self.weapons.insert(weapon.name.clone(), weapon);
    }
    pub fn add_armor(&mut self, armor : Armor) {
        self.armors.insert(armor.name.clone(), armor);
    }
    pub fn get_race(&self, race : &str) -> Option<&Race> {
        self.races.get(race)
    }
    pub fn get_weapon(&self, weapon : &str) -> Option<&Weapon> {
        self.weapons.get(weapon)
    }
    pub fn get_armor(&self, armor : &str) -> Option<&Armor> {
        self.armors.get(armor)
    }
}