use std::collections::{HashSet, HashMap};
use super::attributes::*;
use super::weapons::*;

use serde::{Deserialize, Serialize};

pub struct Character {
    name : String,
    race : Race,
    subrace : String, // Identifier to index into race.subraces
    class : Class,
    subclass : String, // Identifier to index into class.subclasses
    alignment : Alignment,
    attributes : Attributes,
    skill_proficiencies : HashSet<SkillName>,
    weapon_proficiencies : HashSet<WeaponName>,
    weapon_category_proficiencies : HashSet<WeaponCategory>,
    armor_proficiencies : HashSet<ArmorCategory>,
    feats : HashSet<FeatName>,
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

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Copy,Clone)]
#[derive(Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small, 
    Medium,
    Large,
    Huge,
    Gargantuan,
}

type SkillName = String;

pub struct Class {
    name : String,
    save_proficiencies : HashSet<AttributeName>,
    hit_die : DiceSize,
    spells : HashMap<CharacterLevel, HashMap<SpellLevel, i8>>,
    subclasses: HashMap<String, Subclass>,
}

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Subclass {
    name : String,
}

///Type for movement speeds. This is a two-byte int because speeds 
///larger than 255 are rare, but certainly possible.
///Always measured in feet because converting D&D to the metric system
///is more pain than it is worth.
pub type Speed = i16;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Race {
    pub name : String, 
    pub speed : Speed,
    pub size : Size,
    pub attribute_bonuses : AttributeArray,
    pub subraces : HashMap<String, Subrace>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Subrace {
    name : String,
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

#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
#[derive(Copy,Clone)]
pub enum SpellLevel {
    Cantrip,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
}

pub struct Feat {
    name : FeatName,
    text : String,
    effect : Option<Box<Fn(&mut Character)>>,
    undo_effect : Option<Box<Fn(&mut Character)>>,
}

type FeatName = String;

pub type CharacterLevel = i8;

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { 
        Character{ 
            name : name, 
            attributes : Attributes::default(), 
            class : Class::unknown(), 
            subclass : String::new(),
            race : Race::unknown(), 
            subrace : String::new(),
            alignment : Alignment::Neutral,
            feats : HashSet::new(),
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
        if self.subrace == "" { 
            self.attributes.apply_bonuses(&self.race.attribute_bonuses) 
        } else {
        self.attributes
            .apply_bonuses(&self.race.attribute_bonuses)
            .apply_bonuses(&self.race.subraces.get(&self.subrace).unwrap().attribute_bonuses) 
        }
    }
    /// Returns the current attribute modifiers for this character
    pub fn modifiers(&self) -> Attributes { self.attributes.get_modifiers() }
    /// Returns the speed for this character
    pub fn speed(&self) -> Speed { self.race.speed }
    /// Sets the class of this character, resets all prior effects of their class
    pub fn set_class(&mut self, class : Class) { self.class = class }
    /// Sets the class of this character, resets all prior effects of their race
    pub fn set_race(&mut self, race : Race) { self.race = race }
    /// Sets the subrace of this charater to the specified subrace if it exists, 
    /// returns false if it is not a subrace of the current race.
    pub fn set_subrace(&mut self, subrace_name : &str) -> bool {
        if self.race.subraces.contains_key(subrace_name) {
            self.subrace = String::from(subrace_name);
            true
        } else {
            false
        }
    }
    /// Sets the specified attribute of this character to the specified value
    pub fn set_attribute(&mut self, attr : AttributeName, value : AttributeValue) {
        self.attributes.set(attr,value);
    }
    /// Sets the alignment of this character to the specified alignment
    pub fn set_alignment(&mut self, alignment : Alignment) { self.alignment = alignment }
    /// Sets the subclass of this character to the specified subclass if it exists,
    /// returns false if it is not a subclass of the current class
    pub fn set_subclass(&mut self, subclass_name : &str) -> bool { 
        if self.class.subclasses.contains_key(subclass_name) {
            self.subclass = String::from(subclass_name);
            true
        } else {
            false
        } }
    /// Returns the number of spells of the specified spell_level this character can cast at char_level
    pub fn spells_at_level(&self, char_level : CharacterLevel, spell_level : SpellLevel) -> i8 { self.class.spells_at_level(char_level, spell_level) }
    /// Returns the alignment of this character
    pub fn alignment(&self) -> Alignment { self.alignment }
    /// Returns the size of this character
    pub fn size(&self) -> Size { self.race.size }
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
    /// Returns whether or not this character has the specified feat
    pub fn has_feat(&self, feat : &str) -> bool { self.feats.contains(feat) }
    fn remove_feat(&mut self, feat : &str) -> bool { 
        self.feats.remove(feat)
    }
    fn add_feat(&mut self, name : String) {  
        self.feats.insert(name);
    }
}

impl Class {
    fn unknown() -> Self { 
        Class{ 
            name : String::from("UNKNOWN"), 
            subclasses : HashMap::new(),
            save_proficiencies : HashSet::new(),
            spells : HashMap::new(),
            hit_die : DiceSize::D20,
        } 
    }
    fn hit_die(&self) -> DiceSize { self.hit_die }
    fn spells_at_level(&self, char_level : CharacterLevel, spell_level : SpellLevel ) -> i8 {
        println!("{:?}", self.spells.keys());
        match self.spells.get(&char_level) {
            Some(spells) => match spells.get(&spell_level) {
                Some(number_of_spells) => *number_of_spells,
                None => 0,
            }
            None => 0,
        }
    }
}

impl Subclass {
    fn unknown() -> Self {
        Subclass{ name : String::from("UNKNOWN") }
    }
}

impl Race {
    fn unknown() -> Self {
        Race{ name : String::from("UNKNOWN"), speed : 0, attribute_bonuses : HashMap::new(), size : Size::Medium, subraces : HashMap::new() }
    }
}

impl Skill {
    fn new(name : String, attr : AttributeName) -> Self {
        Skill{ name : name, attribute : attr }
    }
}

impl Feat {
    pub fn apply(&self, ch : &mut Character) {
        match &self.effect {
            Some(effect) => effect(ch),
            None => (),
        };
        ch.add_feat(self.name.clone());
    }
    pub fn remove(&self, ch : &mut Character) {
        match &self.undo_effect {
            Some(undo_effect) => undo_effect(ch),
            None => (),
        };
        ch.remove_feat(&self.name);
    }
}

mod test_character;