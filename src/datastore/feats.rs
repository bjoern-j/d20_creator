use super::{ Ability, AbilityScore, Skill };

pub struct Feat {
    pub name : String,
    pub long_text : String,
    pub effects : Vec<Effect>,
    pub prerequisites : Vec<Prerequisite>,
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub enum Effect {
    AbilityIncrease(Ability, AbilityScore),
    SkillProficiency(Skill),
}

pub enum Prerequisite {
    MinimumAbility(Ability, AbilityScore),
}