use std::vec::IntoIter;

#[derive(PartialEq,Eq,Hash)]
#[derive(Copy,Clone)]
#[derive(Debug)]
pub enum Skill {
    Athletics,
    Acrobatics,
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

use Skill::*;

impl Skill {
    pub fn into_iter() -> IntoIter<Skill> {
        vec![
            Athletics,
            Acrobatics,
            SleightOfHand,
            Stealth,
            Arcana,
            History,
            Investigation,
            Nature,
            Religion,
            AnimalHandling,
            Insight,
            Medicine,
            Perception,
            Survival,
            Deception,
            Intimidation,
            Performance,
            Persuasion,
        ].into_iter()
    }
}

#[derive(PartialEq,Eq,Hash)]
#[derive(Copy,Clone)]
#[derive(Debug)]
pub enum SkillLevel {
    None,
    Proficient,
    Expert,
}