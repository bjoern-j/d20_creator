use std::collections::HashMap;
use std::rc::Rc;

mod character;
mod race;
mod size;
mod language;
mod skill;
mod feats;

use size::Size;
use race::Race;
use language::Language;
use character::Character;
use character::attributes::Attribute;
use skill::{Skill, SkillLevel};
use feats::Feat;

type AttributeValue = character::attributes::Value;
type Speed = u16; //u8 is too small since speeds larger than 255 are theoretically possible

pub struct Builder {
    character : Character,
    races : HashMap<String, Race>,
    languages : HashMap<String, Language>,
    feats : HashMap<String, Rc<Feat>>,
}

impl Builder {
    pub fn new() -> Self {
        Builder{
            character : Character::new(),
            races : HashMap::new(),
            languages : HashMap::new(),
            feats : HashMap::new(),
        }
    }
    pub fn set_name(&mut self, name : String) {
        self.character.name = Some(name);        
    }
    pub fn character(&self) -> &Character {
        &self.character
    }
    pub fn set_attribute(&mut self, attribute : Attribute, value : AttributeValue) {
        self.character.attributes.insert(attribute, value);
    }
    pub fn add_race(&mut self, race : Race) {
        self.races.insert(race.name().to_owned(), race);
    }
    pub fn set_race(&mut self, race : &str) {
        self.unset_race();
        self.character.race = Some(race.to_owned());
        let new_race = self.races.get(race).unwrap();
        for (attr, val) in new_race.attributes.iter() {
            let char_attr = self.character.attributes.get_mut(attr).unwrap();
            *char_attr += val;
        };
        self.character.size = Some(new_race.size);
        self.character.speed = Some(new_race.speed);
    }
    pub fn add_language(&mut self, language : Language) {
        self.languages.insert(language.name.clone(), language);
    }
    pub fn add_feat(&mut self, feat : Rc<Feat>) {
        self.feats.insert(feat.name().to_owned(), feat);
    }
    pub fn add_feat_to_character(&mut self, feat : &str) {
        self.character.feats.insert(feat.to_owned());
    }
    pub fn add_character_language(&mut self, language : &str) {
        self.character.languages.insert(language.to_owned());
    }
    pub fn set_skill_level(&mut self, skill : Skill, level : SkillLevel) {
        self.character.skills.insert(skill, level);
    }
    fn unset_race(&mut self) {
        match &self.character.race {
            Some(race) => 
                for (attr, val) in self.races.get(race).unwrap().attributes.iter() {
                    let char_attr = self.character.attributes.get_mut(attr).unwrap();
                    *char_attr -= val;
                },
            None => (),
        }
        self.character.race = None;
        self.character.size = None;
    }
}

#[cfg(test)]
mod test_builder_1;