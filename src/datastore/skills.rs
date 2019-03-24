use super::{ WeaponCategory, ArmorCategory };

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SkillLevel { None, Proficient, Expert }

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum CombatProficiency{
    Weapon(String),
    WeaponCategory(WeaponCategory),
    ArmorCategory(ArmorCategory),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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