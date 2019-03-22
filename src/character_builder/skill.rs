use std::vec::IntoIter;

#[derive(PartialEq,Eq,Hash)]
#[derive(Clone)]
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
    AlchemistTools,
    BrewerTools,
    CalligrapherTools,
    CarpenterTools,
    CartographerTools,
    CobblerTools,
    CookTools,
    GlassblowerTools,
    JewelerTools,
    LeatherworkerTools,
    MasonTools,
    PainterTools,
    PotterTools,
    SmithTools,
    TinkerTools,
    WeaverTools,
    WoodcarverTools,
    DisguiseTools,
    ForgeryTools,
    HerbalistTools,
    NavigatorTools,
    PoisonerTools,
    ThievesTools,
    GamingTools(String),
    MusicalInstrument(String),
    Vehicle(String),
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