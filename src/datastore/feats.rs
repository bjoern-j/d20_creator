use super::{ Ability, AbilityScore };

pub struct Feat {
    pub name : String,
    pub long_text : String,
    pub effects : Vec<Effect>,
    pub prerequisite : Prerequisite,
}

pub enum Effect {
    None,
    AbilityIncrease(Ability, AbilityScore),
}

pub enum Prerequisite {
    None,
}