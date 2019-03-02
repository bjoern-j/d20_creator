use std::collections::HashSet;
use std::iter::FromIterator;

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

type AttributeValue = i8;

#[derive(PartialEq,Eq,Hash)]
#[derive(Copy,Clone)]
pub enum AttributeName {
    Str,
    Dex,
    Con,
    Wis,
    Int,
    Cha,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub struct Attributes {
    str : AttributeValue,
    dex : AttributeValue,
    con : AttributeValue,
    wis : AttributeValue,
    int : AttributeValue,
    cha : AttributeValue,
}

impl Attributes {
    /// Returns the average stats array with all zero modifiers
    fn default() -> Self { Attributes{ str : 10, dex : 10, con : 10, wis : 10, int : 10, cha : 10 } }
    /// Returns the modifiers for these attributes
    fn get_modifiers(&self) -> Self {
        Attributes { 
            str : Self::compute_mod(self.str),
            dex : Self::compute_mod(self.dex),
            con : Self::compute_mod(self.con),
            wis : Self::compute_mod(self.wis),
            int : Self::compute_mod(self.int),
            cha : Self::compute_mod(self.cha), }
    }
    /// Computes the modifier for a given attribute value. 
    /// Not a one-line "(value - 10)/2" because Rust rounds towards zero, 
    /// but we want flooring also for negative values.
    fn compute_mod(value : AttributeValue) -> AttributeValue { 
        let double_mod = value - 10;
        if double_mod > 0 {
            double_mod / 2
        } else {
            (double_mod - 1) / 2
        }
    }
    fn get(&self, attr : AttributeName) -> AttributeValue {
        match attr {
            AttributeName::Str => self.str,
            AttributeName::Dex => self.dex,
            AttributeName::Con => self.con,
            AttributeName::Int => self.int, 
            AttributeName::Wis => self.wis,
            AttributeName::Cha => self.cha,
        }
    }
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
        assert_eq!(ch.modifiers(), Attributes{ str:0, dex:0, con:0, wis:0, int:0, cha:0 } );
    }
    #[test]
    fn test_modifiers() {
        let mut ch = Character::new(String::from("Babe"));
        ch.set_attributes( Attributes { str:9, dex:10, con:11, wis:12, int:13, cha:7 } );
        assert_eq!(ch.modifiers(), Attributes { str:-1, dex:0, con:0, wis:1, int:1, cha:-2 } );
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