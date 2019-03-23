use super::*;

pub struct Class {
    pub name : String,
    pub hit_die : Die,
    pub weapon_and_armor_proficiencies : Vec<WeaponOrArmor>,
    pub skill_proficiencies : Vec<Skill>,
}

impl Class {
    pub fn name(&self) -> &str { &self.name }
}