use std::collections::HashMap;
use crate::character::{ Ability, AbilityScore, Size, Speed, Die };

type AbilityArray = HashMap<Ability, AbilityScore>;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponCategory { Simple, Martial }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WeaponRange{ Melee, Ranged }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ArmorCategory { Light, Medium, Heavy, Shield }
pub type Reach = u16;


mod races;
pub use races::{ Race, Subrace };

mod spells;
pub use spells::{ Spell, SpellComponent, SpellSchool, SpellLevel, SpellCaster, SpellSlots };

mod skills;
pub use skills::{ Skill, SkillLevel, CombatProficiency };

mod feats;
pub use feats::{ Feat, Effect as FeatEffect, Prerequisite as FeatPrerequisite };

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
    feats : HashMap<String, Feat>,
    spells : HashMap<String, Spell>,
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
            feats : HashMap::new(),
            spells : HashMap::new(),
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
    pub fn add_feat(&mut self, feat : Feat) {
        self.feats.insert(feat.name.clone(), feat);
    }
    pub fn add_spell(&mut self, spell : Spell) {
        self.spells.insert(spell.name.clone(), spell);
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
    pub fn get_feat(&self, feat : &str) -> Option<&Feat> {
        self.feats.get(feat)
    }
    pub fn get_spell(&self, spell : &str) -> Option<&Spell> {
        self.spells.get(spell)
    }
}

