pub mod attributes;
use attributes::Attribute;
use super::{HashMap, Size, Speed, Skill, SkillLevel, WeaponCategory, ArmorCategory, Die, Modifier};
use std::collections::HashSet;

type CharacterLevel = i8; //Signed because it is used in computations that return signed values

pub struct Character {
    name : Option<String>,
    level : CharacterLevel,
    attributes : HashMap<attributes::Attribute, attributes::Value>,
    race : Option<String>,
    subrace : Option<String>,
    size : Option<Size>,
    speed : Option<Speed>,
    class : Option<String>,
    hit_die : Option<Die>,
    languages : HashSet<String>,
    skills : HashMap<Skill, SkillLevel>,
    feats : HashSet<String>,
    spells : HashMap<String, Attribute>,
    weapon_category_proficiencies : HashSet<WeaponCategory>,
    armor_proficiencies : HashSet<ArmorCategory>,
    weapon_proficiencies : HashSet<String>,
    saving_throws : HashSet<Attribute>,
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
            level : 1,
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
            saving_throws : HashSet::new(),
        }
    }
    pub fn name(&self) -> &str { 
        match &self.name {
            None => "",
            Some(name) => &name,
        }
    }
    pub fn attribute(&self, attribute : &Attribute) -> attributes::Value {
        match self.attributes.get(attribute) {
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
    pub fn saving_throw_mod(&self, attribute : &Attribute) -> Modifier {
        Attribute::modifier(self.attribute(attribute)) 
        +
        if self.saving_throws.contains(&attribute) { self.proficiency_bonus() } else { 0 }
    }
    pub fn proficiency_bonus(&self) -> Modifier {
        2 + (self.level / 4)
    }
    pub fn race(&self) -> &Option<String> {
        &self.race
    }
    pub fn subrace(&self) -> &Option<String> {
        &self.subrace
    }
    pub(super) fn set_name(&mut self, name : String) {
        self.name = Some(name);
    }
    pub(super) fn set_attribute(&mut self, attribute : Attribute, value : attributes::Value) {
        self.attributes.insert(attribute, value);
    }
    pub(super) fn set_class(&mut self, class : String) {
        self.class = Some(class);
    }
    pub(super) fn set_hit_die(&mut self, die : Die) {
        self.hit_die = Some(die);
    }
    pub(super) fn set_race(&mut self, race : String) {
        self.race = Some(race);
    }
    pub(super) fn set_size(&mut self, size : Size) {
        self.size = Some(size);
    }
    pub(super) fn set_speed(&mut self, speed : Speed) {
        self.speed = Some(speed);
    }
    pub(super) fn set_subrace(&mut self, subrace : String) {
        self.subrace = Some(subrace);
    }
    pub(super) fn get_mut_attribute(&mut self, attr : &Attribute) -> &mut attributes::Value {
        self.attributes.get_mut(attr).unwrap()
    }
    pub(super) fn add_spell(&mut self, spell : String, attr : Attribute) {
        self.spells.insert(spell, attr);
    }
    pub(super) fn add_language(&mut self, language : String) {
        self.languages.insert(language);
    }
    pub(super) fn add_feat(&mut self, feat : String) {
        self.feats.insert(feat);
    }
    pub(super) fn remove_feat(&mut self, feat : &str) {
        self.feats.remove(feat);
    }
    pub(super) fn clear_race(&mut self) {
        self.race = None;
    }
    pub(super) fn clear_size(&mut self) {
        self.size = None;
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
    pub(super) fn add_saving_throw_proficiency(&mut self, attr : Attribute) {
        self.saving_throws.insert(attr);
    }
}