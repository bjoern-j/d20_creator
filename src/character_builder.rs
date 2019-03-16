use std::collections::HashMap;

mod character;
mod race;
mod size;

use size::Size;
use race::Race;
use character::Character;
use character::attributes::Attribute;

type AttributeValue = character::attributes::Value;
type Speed = u16; //u8 is too small since speeds larger than 255 are theoretically possible

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
    pub fn unset_race(&mut self) {
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