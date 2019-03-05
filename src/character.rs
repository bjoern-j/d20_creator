use std::collections::{HashSet, HashMap};
use super::attributes::*;
use super::weapons::*;

use serde::{Deserialize, Serialize};

pub struct Character {
    name : String,
    race : Race,
    class : Class,
    alignment : Alignment,
    attributes : Attributes,
    skill_proficiencies : HashSet<SkillName>,
    weapon_proficiencies : HashSet<WeaponName>,
    weapon_category_proficiencies : HashSet<WeaponCategory>,
    armor_proficiencies : HashSet<ArmorCategory>,
}

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Copy,Clone)]
pub enum ArmorCategory{
    Light,
    Medium,
    Heavy,
    Shield,
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

#[derive(Serialize, Deserialize)]
pub struct Race {
    pub name : String, 
    pub speed : Speed,
    pub attribute_bonuses : AttributeArray,
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

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Copy,Clone)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    Neutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { 
        Character{ 
            name : name, 
            attributes : Attributes::default(), 
            class : Class::unknown(), 
            race : Race::unknown(), 
            alignment : Alignment::Neutral,
            skill_proficiencies : HashSet::new(),
            weapon_proficiencies : HashSet::new(),
            weapon_category_proficiencies : HashSet::new(),
            armor_proficiencies : HashSet::new(),
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
    /// Sets the alignment of this character to the specified alignment
    pub fn set_alignment(&mut self, alignment : Alignment) { self.alignment = alignment }
    /// Returns the alignment of this character
    pub fn alignment(&self) -> Alignment { self.alignment }
    /// Returns the modifier of this character for the specified skill
    pub fn skill_mod(&self, skill : &Skill) -> AttributeValue {
        self.modifiers().get(skill.attribute) + if self.skill_proficiencies.contains(&skill.name) { self.proficiency_bonus() } else { 0 }
    }
    /// Makes this character proficient in the specified skill
    pub fn add_skill_proficiency(&mut self, skill : &Skill) {
        self.skill_proficiencies.insert(skill.name.clone());
    }
    /// Makes this character proficient with the specified type of armor
    pub fn add_armor_proficiency(&mut self, armor_category : ArmorCategory) {
        self.armor_proficiencies.insert(armor_category);
    }
    /// Returns whether this character is proficient with the specified type of armor
    pub fn proficient_with(&self, armor_category : ArmorCategory) -> bool {
        self.armor_proficiencies.contains(&armor_category)
    }
    /// Returns the proficiency bonus of this characters
    pub fn proficiency_bonus(&self) -> AttributeValue { 2 }
    /// Returns the current modifier for the saving throw of this character for the specified attribute
    pub fn save_mod(&self, attr : AttributeName) -> AttributeValue {
        let mods = self.modifiers();
        mods.get(attr) + if self.class.save_proficiencies.contains(&attr) { self.proficiency_bonus() } else { 0 }
    }
    /// Returns this characters attack modifier with the specified weapon
    pub fn weapon_attack_mod(&self, weapon : &Weapon) -> AttributeValue {
        self.modifiers().get(match weapon.weapon_type() {
            WeaponType::Melee  => AttributeName::Str,
            WeaponType::Ranged => AttributeName::Dex,
        }) 
        + 
        if self.weapon_category_proficiencies.contains(&weapon.category()) 
        || self.weapon_proficiencies.contains(&weapon.name()) { 
            self.proficiency_bonus() 
        } else { 0 }
    }
    /// Makes this character proficient with the specified weapon
    pub fn add_weapon_proficiency(&mut self, weapon : &Weapon) {
        self.weapon_proficiencies.insert(weapon.name().clone());
    }
    /// Makes this character proficient with the specified weapon category
    pub fn add_weapon_category_proficiency(&mut self, category : WeaponCategory) {
        self.weapon_category_proficiencies.insert(category);
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

mod test_character;