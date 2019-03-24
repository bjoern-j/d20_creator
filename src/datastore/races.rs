use super::{ AbilityArray, Skill, CombatProficiency, Speed, Size };
use std::collections::HashMap;

pub struct Race {
    pub name : String,
    pub long_text : String,
    pub ability_bonuses : AbilityArray,
    pub size : Size,
    pub speed : Speed,
    pub languages: Vec<String>,
    pub skill_proficiencies : Vec<Skill>,
    pub combat_proficiencies : Vec<CombatProficiency>,
    pub subraces : HashMap<String, Subrace>,
}

pub struct Subrace {
    pub name : String,
    pub long_text : String,
    pub ability_bonuses : AbilityArray,
    pub languages : Vec<String>,
    pub skill_proficiencies : Vec<Skill>,
    pub combat_proficiencies : Vec<CombatProficiency>,
}

impl Race {
    pub fn add_subrace(&mut self, subrace : Subrace) {
        self.subraces.insert(subrace.name.clone(), subrace);
    }
    pub fn get_subrace(&self, subrace : &str) -> Option<&Subrace> {
        self.subraces.get(subrace)
    }
}