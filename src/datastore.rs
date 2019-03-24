use std::collections::HashMap;
use crate::character::{ Ability, AbilityScore, Size, Speed, Skill, CombatProficiency, Die };
use std::iter::FromIterator;

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

type AbilityArray = HashMap<Ability, AbilityScore>;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponCategory { Simple, Martial }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponRange{ Melee, Ranged }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ArmorCategory { Light, Medium, Heavy, Shield }
pub type Reach = u16;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpellCaster { None, Third, Half, Full }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpellLevel { Cantrip, First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth, Ninth }
pub type SpellSlots = HashMap<SpellLevel, u8>;


impl SpellLevel {
    pub fn slots(first : u8, second : u8, third : u8, fourth : u8, fifth : u8, sixth : u8, seventh : u8, eighth : u8, ninth : u8) -> SpellSlots {
        HashMap::from_iter(
            Iterator::zip(
                vec![SpellLevel::First, SpellLevel::Second, SpellLevel::Third, 
                     SpellLevel::Fourth, SpellLevel::Fifth, SpellLevel::Sixth, 
                     SpellLevel::Seventh, SpellLevel::Eighth, SpellLevel::Ninth].iter().cloned(),
                vec![first, second, third,
                     fourth, fifth, sixth,
                     seventh, eighth, ninth].iter().cloned()
            )
        )
    }
    pub fn slots_for_level(level : &i8, caster : &SpellCaster) -> SpellSlots {
        match caster {
            SpellCaster::None => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
            SpellCaster::Full => match level {
                1 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                6 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                8 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                9 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                10 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                11 => SpellLevel::slots(4,3,3,3,2,1,0,0,0),
                12 => SpellLevel::slots(4,3,3,3,2,1,0,0,0),
                13 => SpellLevel::slots(4,3,3,3,2,1,1,0,0),
                14 => SpellLevel::slots(4,3,3,3,2,1,1,0,0),
                15 => SpellLevel::slots(4,3,3,3,2,1,1,1,0),
                16 => SpellLevel::slots(4,3,3,3,2,1,1,1,0),
                17 => SpellLevel::slots(4,3,3,3,2,1,1,1,1),
                18 => SpellLevel::slots(4,3,3,3,3,1,1,1,1),
                19 => SpellLevel::slots(4,3,3,3,3,2,1,1,1),
                20 => SpellLevel::slots(4,3,3,3,3,2,2,1,1),
                _ => panic!("Invalid level")
            },
            SpellCaster::Half => match level {
                1 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                6 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                8 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                9 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                10 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                11 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                12 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                13 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                14 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                15 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                16 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                17 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                18 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                19 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                20 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                _ => panic!("Invalid level")
            },
            SpellCaster::Third => match level {
                1 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                6 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                8 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                9 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                10 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                11 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                12 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                13 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                14 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                15 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                16 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                17 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                18 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                19 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                20 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                _ => panic!("Invalid level")
            },
        }
    }
}

pub struct Weapon {
    pub name : String,
    pub category : WeaponCategory,
    pub range_category : WeaponRange,
    pub reach : Reach,
}

pub struct Armor {
    pub name : String,
    pub category : ArmorCategory,
}

pub struct Datastore {
    races : HashMap<String, Race>,
    weapons : HashMap<String, Weapon>,
    armors : HashMap<String, Armor>,
    classes : HashMap<String, Class>,
}

pub struct Class {
    pub name : String,
    pub long_text : String,
    pub hit_die : Die,
    pub saving_throws : Vec<Ability>,
    pub combat_proficiencies : Vec<CombatProficiency>,
    pub skill_proficiencies : Vec<Skill>,
    pub spell_caster : SpellCaster,
}

impl Datastore {
    /// Creates a new data store without any data in it
    pub fn new() -> Self { 
        Datastore {
            races : HashMap::new(),
            weapons : HashMap::new(),
            armors : HashMap::new(),
            classes : HashMap::new(),
        }
    }
    pub fn add_race(&mut self, race : Race) {
        self.races.insert(race.name.clone(), race);
    }
    pub fn add_weapon(&mut self, weapon : Weapon) {
        self.weapons.insert(weapon.name.clone(), weapon);
    }
    pub fn add_armor(&mut self, armor : Armor) {
        self.armors.insert(armor.name.clone(), armor);
    }
    pub fn add_class(&mut self, class : Class) {
        self.classes.insert(class.name.clone(), class);
    }
    pub fn get_race(&self, race : &str) -> Option<&Race> {
        self.races.get(race)
    }
    pub fn get_weapon(&self, weapon : &str) -> Option<&Weapon> {
        self.weapons.get(weapon)
    }
    pub fn get_armor(&self, armor : &str) -> Option<&Armor> {
        self.armors.get(armor)
    }
    pub fn get_class(&self, class : &str) -> Option<&Class> {
        self.classes.get(class)
    }
}

impl Race {
    pub fn add_subrace(&mut self, subrace : Subrace) {
        self.subraces.insert(subrace.name.clone(), subrace);
    }
    pub fn get_subrace(&self, subrace : &str) -> Option<&Subrace> {
        self.subraces.get(subrace)
    }
}