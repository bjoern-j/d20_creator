pub mod attributes;
use attributes::Attribute;
use super::{HashMap, Size, Speed, Skill, SkillLevel, WeaponCategory, ArmorCategory, Die};
use std::collections::HashSet;

pub struct Character {
    pub(super) name : Option<String>,
    pub(super) attributes : HashMap<attributes::Attribute, attributes::Value>,
    pub(super) race : Option<String>,
    pub(super) subrace : Option<String>,
    pub(super) size : Option<Size>,
    pub(super) speed : Option<Speed>,
    pub(super) class : Option<String>,
    pub(super) hit_die : Option<Die>,
    pub(super) languages : HashSet<String>,
    pub(super) skills : HashMap<Skill, SkillLevel>,
    pub(super) feats : HashSet<String>,
    pub(super) spells : HashMap<String, Attribute>,
    pub(super) weapon_category_proficiencies : HashSet<WeaponCategory>,
    pub(super) armor_proficiencies : HashSet<ArmorCategory>,
    pub(super) weapon_proficiencies : HashSet<String>,
}

pub enum WeaponOrArmor {
    WeaponCategory(WeaponCategory),
    ArmorCategory(ArmorCategory),
    Weapon(String),
}


impl Character {
    pub(super) fn new() -> Self {
        Character {
            name : None,
            attributes : Attribute::array_of_ten(),
            race : None,
            subrace : None,
            size : None,
            speed : None,
            class : None,
            hit_die : None,
            languages : HashSet::new(),
            skills : HashMap::new(),
            feats : HashSet::new(),
            spells : HashMap::new(),
            weapon_category_proficiencies : HashSet::new(),
            armor_proficiencies : HashSet::new(),
            weapon_proficiencies : HashSet::new(),
        }
    }
    pub fn name(&self) -> &str { 
        match &self.name {
            None => "",
            Some(name) => &name,
        }
    }
    pub fn attribute(&self, attribute : Attribute) -> attributes::Value {
        match self.attributes.get(&attribute) {
            None => 0,
            Some(val) => *val,
        }
    }
    pub fn size(&self) -> Size {
        match &self.size {
            Some(size) => *size,
            None => Size::Medium,
        }
    }
    pub fn speed(&self) -> Speed {
        match &self.speed {
            Some(speed) => *speed,
            None => 30,
        }
    }
    pub fn speaks(&self, language : &str) -> bool {
        self.languages.contains(language)
    }
    pub fn skill_level(&self, skill : &Skill) -> SkillLevel {
        match self.skills.get(skill) {
            Some(level) => *level,
            None => SkillLevel::None,
        }
    }
    pub fn has_feat(&self, name : &str) -> bool {
        self.feats.contains(name)
    }
    pub fn proficient_with(&self, weapon_or_armor : &WeaponOrArmor) -> bool {
        match weapon_or_armor {
            WeaponOrArmor::WeaponCategory(cat) => self.weapon_category_proficiencies.contains(cat),
            WeaponOrArmor::ArmorCategory(cat) => self.armor_proficiencies.contains(cat),
            WeaponOrArmor::Weapon(weapon) => self.weapon_proficiencies.contains(weapon),
        }
    }
    pub fn knows_spell(&self, spell : &str) -> bool {
        self.spells.contains_key(spell)
    }
    pub fn spellcasting_ability(&self, spell : &str) -> Attribute {
        *self.spells.get(spell).unwrap()
    }
    pub fn hit_die(&self) -> Die {
        match &self.hit_die {
            Some(die) => *die,
            None => Die::D20,
        }
    }
    pub(super) fn add_weapon_or_armor_proficiency(&mut self, prof : &WeaponOrArmor) {
        match prof {
            WeaponOrArmor::WeaponCategory(cat) => { self.weapon_category_proficiencies.insert(*cat); },
            WeaponOrArmor::ArmorCategory(cat) => { self.armor_proficiencies.insert(*cat); },
            WeaponOrArmor::Weapon(weapon) => { self.weapon_proficiencies.insert(weapon.to_owned()); }
        }        
    }
    pub(super) fn set_skill_level(&mut self, skill : &Skill, level : SkillLevel) {
        self.skills.insert(skill.clone(), level);
    }
}