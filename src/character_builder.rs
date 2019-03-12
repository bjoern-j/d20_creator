use std::collections::HashMap;

mod character;

type Character = character::Character;
type AttributeValue = character::attributes::Value;
type Attribute = character::attributes::Attribute;

pub struct Builder {
    character : Character,
}

impl Builder {
    pub fn new() -> Self {
        Builder{
            character : Character::new(),
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
}

#[cfg(test)]
mod test_builder_1;