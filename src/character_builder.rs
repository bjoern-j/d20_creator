use std::collections::HashMap;

mod character;
mod race;

use race::Race;
use character::Character;
use character::attributes::Attribute;

type AttributeValue = character::attributes::Value;

pub struct Builder {
    character : Character,
    races : HashMap<String, Race>,
}

impl Builder {
    pub fn new() -> Self {
        Builder{
            character : Character::new(),
            races : HashMap::new(),
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
        self.character.race = Some(race.to_owned());
        for (attr, val) in self.races.get(race).unwrap().attributes.iter() {
            let char_attr = self.character.attributes.get_mut(attr).unwrap();
            *char_attr += val;
        };
    }
}

#[cfg(test)]
mod test_builder_1;