use std::collections::HashSet;
use std::iter::FromIterator;
use super::attributes::*;

pub struct Character {
    name : String,
    class : Class,
    pub attributes : Attributes,
}

pub struct Class {
    name : String,
    save_proficiencies : HashSet<AttributeName>,
    subclass: Subclass,
}

pub struct Subclass {
    name : String,
}

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { Character{ name : name, attributes : Attributes::default(), class : Class::unknown() } }
    /// Returns the name of the character as a string so it can be manipulated
    /// and stored without holding a reference to the character themself.
    pub fn name(&self) -> String { self.name.clone() }
    /// Sets the whole array of attributes at once
    pub fn set_attributes(&mut self, attrs : Attributes) { self.attributes = attrs }
    /// Returns the current attribute modifiers for this character
    pub fn modifiers(&self) -> Attributes { self.attributes.get_modifiers() }
    /// Sets the class of this character, resets all prior effects of their class
    pub fn set_class(&mut self, class : Class) { self.class = class }
    /// Returns the current modifier for the saving throw of this character for the specified attribute
    pub fn save_mod(&self, attr : AttributeName) -> AttributeValue {
        let mods = self.modifiers();
        mods.get(attr) + if self.class.save_proficiencies.contains(&attr) { 2 } else { 0 }
    }
}

impl Class {
    fn unknown() -> Self { 
        Class{ 
            name : String::from("UNKNOWN"), 
            subclass : Subclass::unknown(),
            save_proficiencies : HashSet::new(),
        } 
    }
}

impl Subclass {
    fn unknown() -> Self {
        Subclass{ name : String::from("UNKNOWN") }
    }
}

#[cfg(test)]
mod test_character {
    use super::*;
    #[test]
    fn test_create_with_name() {
        let ch = Character::new(String::from("Dude"));
        assert_eq!(ch.name(), "Dude");
    }
    #[test]
    fn test_set_attributes() {
        let mut ch = Character::new(String::from("Gal"));
        ch.set_attributes( Attributes::default() );
        assert_eq!(ch.attributes, Attributes::default());
    }
    #[test]
    fn test_get_modifiers() {
        let ch = Character::new(String::from("Hunk"));
        assert_eq!(ch.modifiers(), Attributes::new(0,0,0,0,0,0) );
    }
    #[test]
    fn test_modifiers() {
        let mut ch = Character::new(String::from("Babe"));
        ch.set_attributes( Attributes::new(9,10,11,12,13,7) );
        assert_eq!(ch.modifiers(), Attributes::new(-1,0,0,1,1,-2) );
    }
    #[test]
    fn test_set_class() {
        let warrior = Class{ 
            name : String::from("Warrior"), 
            save_proficiencies : HashSet::from_iter([AttributeName::Str, AttributeName::Con].iter().cloned()),  
            subclass : Subclass::unknown(),
        };
        let mut ch = Character::new(String::from("Vala"));
        ch.set_class(warrior);
        assert_eq!(ch.save_mod(AttributeName::Str), 2);
    }
}