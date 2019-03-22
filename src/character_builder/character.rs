pub mod attributes;
use super::{HashMap, Size, Speed, Skill, SkillLevel};
use std::collections::HashSet;

pub struct Character {
    pub(super) name : Option<String>,
    pub(super) attributes : HashMap<attributes::Attribute, attributes::Value>,
    pub(super) race : Option<String>,
    pub(super) size : Option<Size>,
    pub(super) speed : Option<Speed>,
    pub(super) languages : HashSet<String>,
    pub(super) skills : HashMap<Skill, SkillLevel>,
    pub(super) feats : HashSet<String>,
}

impl Character {
    pub(super) fn new() -> Self {
        Character {
            name : None,
            attributes : attributes::Attribute::array_of_ten(),
            race : None,
            size : None,
            speed : None,
            languages : HashSet::new(),
            skills : HashMap::new(),
            feats : HashSet::new(),
        }
    }
    pub fn name(&self) -> &str { 
        match &self.name {
            None => "",
            Some(name) => &name,
        }
    }
    pub fn attribute(&self, attribute : attributes::Attribute) -> attributes::Value {
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
}