use std::collections::{HashSet, HashMap};
use super::attributes::*;

pub struct Character {
    name : String,
    race : Race,
    class : Class,
    attributes : Attributes,
    skill_proficiencies : HashSet<SkillName>,
}

type SkillName = String;

pub struct Class {
    name : String,
    save_proficiencies : HashSet<AttributeName>,
    hit_die : DiceSize,
    subclass: Subclass,
}

pub struct Subclass {
    name : String,
}

///Type for movement speeds. This is a two-byte int because speeds 
///larger than 255 are rare, but certainly possible.
///Always measured in feet because converting D&D to the metric system
///is more pain than it is worth.
pub type Speed = i16;

pub struct Race {
    name : String, 
    speed : Speed,
    attribute_bonuses : AttributeArray,
}

pub struct Skill {
    name : String,
    attribute : AttributeName,
}

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Copy,Clone)]
pub enum DiceSize {
    D4, D6, D8, D10, D12, D20,
}

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { 
        Character{ 
            name : name, 
            attributes : Attributes::default(), 
            class : Class::unknown(), 
            race : Race::unknown(), 
            skill_proficiencies : HashSet::new(),
        } 
    }
    /// Returns the name of the character as a string so it can be manipulated
    /// and stored without holding a reference to the character themself.
    pub fn name(&self) -> String { self.name.clone() }
    /// Sets the whole array of attributes at once
    pub fn set_attributes(&mut self, attrs : Attributes) { self.attributes = attrs }
    /// Returns the full array of attributes of this character
    pub fn attributes(&self) -> Attributes { 
        self.attributes.apply_bonuses(&self.race.attribute_bonuses)
    }
    /// Returns the current attribute modifiers for this character
    pub fn modifiers(&self) -> Attributes { self.attributes.get_modifiers() }
    /// Returns the speed for this character
    pub fn speed(&self) -> Speed { self.race.speed }
    /// Sets the class of this character, resets all prior effects of their class
    pub fn set_class(&mut self, class : Class) { self.class = class }
    /// Sets the class of this character, resets all prior effects of their race
    pub fn set_race(&mut self, race : Race) { self.race = race }
    /// Sets the specified attribute of this character to the specified value
    pub fn set_attribute(&mut self, attr : AttributeName, value : AttributeValue) {
        self.attributes.set(attr,value);
    }
    /// Returns the modifier of this character for the specified skill
    pub fn skill_mod(&self, skill : &Skill) -> AttributeValue {
        self.modifiers().get(skill.attribute) + if self.skill_proficiencies.contains(&skill.name) { self.proficiency_bonus() } else { 0 }
    }
    /// Makes this character proficient in the specified skill
    pub fn add_skill_proficiency(&mut self, skill : &Skill) {
        self.skill_proficiencies.insert(skill.name.clone());
    }
    /// Returns the proficiency bonus of this characters
    pub fn proficiency_bonus(&self) -> AttributeValue { 2 }
    /// Returns the current modifier for the saving throw of this character for the specified attribute
    pub fn save_mod(&self, attr : AttributeName) -> AttributeValue {
        let mods = self.modifiers();
        mods.get(attr) + if self.class.save_proficiencies.contains(&attr) { self.proficiency_bonus() } else { 0 }
    }
    /// Returns the current hit die of this character
    pub fn hit_die(&self) -> DiceSize { self.class.hit_die() }
}

impl Class {
    fn unknown() -> Self { 
        Class{ 
            name : String::from("UNKNOWN"), 
            subclass : Subclass::unknown(),
            save_proficiencies : HashSet::new(),
            hit_die : DiceSize::D20,
        } 
    }
    fn hit_die(&self) -> DiceSize { self.hit_die }
}

impl Subclass {
    fn unknown() -> Self {
        Subclass{ name : String::from("UNKNOWN") }
    }
}

impl Race {
    fn unknown() -> Self {
        Race{ name : String::from("UNKNOWN"), speed : 0, attribute_bonuses : HashMap::new() }
    }
}

impl Skill {
    fn new(name : String, attr : AttributeName) -> Self {
        Skill{ name : name, attribute : attr }
    }
}

#[cfg(test)]
mod test_character {
    use super::*;
    use std::iter::FromIterator;
    #[test]
    fn test_create_with_name() {
        let ch = Character::new(String::from("Dude"));
        assert_eq!(ch.name(), "Dude");
    }
    #[test]
    fn test_set_attributes() {
        let mut ch = Character::new(String::from("Gal"));
        ch.set_attributes( Attributes::default() );
        assert_eq!(ch.attributes(), Attributes::default());
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
        let ch = get_warrior();
        assert_eq!(ch.save_mod(AttributeName::Str), 2);
    }
    #[test]
    fn test_hit_die() {
        let ch = get_warrior();
        assert_eq!(ch.hit_die(), DiceSize::D10);
    }
    #[test]
    fn test_set_race() {
        let ch = get_orc();
        assert_eq!(ch.attributes(), Attributes::new(12,10,11,10,10,10));
    }
    #[test]
    fn test_speed() {
        let ch = get_orc();
        assert_eq!(ch.speed(), 35);
    }
    #[test]
    fn test_skills() {
        let eat = Skill::new(String::from("Eat"), AttributeName::Con);
        let mut ch = Character::new(String::from("Munchie"));
        ch.set_attribute(AttributeName::Con, 16);
        assert_eq!(ch.skill_mod(&eat), 3);
        ch.add_skill_proficiency(&eat);
        assert_eq!(ch.skill_mod(&eat), 5);
    }

    fn get_warrior() -> Character {
        let warrior = Class{ 
            name : String::from("Warrior"), 
            save_proficiencies : HashSet::from_iter([AttributeName::Str, AttributeName::Con].iter().cloned()),  
            hit_die : DiceSize::D10,
            subclass : Subclass::unknown(),
        };
        let mut ch = Character::new(String::from("Vala"));
        ch.set_class(warrior);
        ch
    }
    fn get_orc() -> Character {
        let orc = Race{
            name : String::from("Orc"),
            speed : 35,
            attribute_bonuses : HashMap::from_iter([(AttributeName::Str, 2), (AttributeName::Con, 1)].iter().cloned()),
        };
        let mut ch = Character::new(String::from("Hruumsh"));
        ch.set_race(orc);
        ch
    }
}