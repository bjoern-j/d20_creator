use crate::datastore::{ Datastore, Weapon, WeaponCategory, WeaponRange, Armor, ArmorCategory, Race };
use std::collections::{ HashMap, HashSet };

pub struct Character<'d> {
    pub name : String,
    data : &'d Datastore,
    abilities : Abilities,
    race : String,
    languages : HashSet<String>,
    skills : HashMap<Skill, SkillLevel>,
    combat_proficiencies : HashSet<CombatProficiency>,
}

impl<'d> Character<'d> {
    /// Creates a new character that will draw on the data in the store passed to it here.
    /// This means characters cannot outlive the existence of the data store, which is reasonable
    /// since they need the data in it to know what e.g. feats do.
    pub fn new(data : &'d Datastore) -> Self {
        Character {
            name : String::new(),
            data : data,
            abilities : Abilities::new(),
            race : String::new(),
            languages : HashSet::new(),
            skills : HashMap::new(),
            combat_proficiencies : HashSet::new(),
        }
    }
    /// Returns the current ability score of the character for the ability
    pub fn ability(&self, ability : &Ability) -> &AbilityScore {
        self.abilities.get(ability)
    }
    /// Returns the current size of the character, or throws an error if they have no race determining their size
    pub fn size(&self) -> Result<&Size, String> {
        match self.data.get_race(&self.race) {
            Some(r) => Ok(&r.size),
            None => Err("Character has no race or race was not found.".to_owned())
        }
    }
    /// Endows the character with the ability to speak the specified language
    pub fn learn_language(&mut self, language : String) {
        self.languages.insert(language);
    }
    /// Makes the character proficient in a weapon, weapon category or armor category
    pub fn add_combat_proficiency(&mut self, prof : CombatProficiency) {
        self.combat_proficiencies.insert(prof);
    }
    /// Returns the attack modifier of the character with the specified weapon,
    /// taking into account proficiencies
    pub fn get_attack_mod(&self, weapon : &Weapon) -> Modifier {
        let ability_mod = match weapon.range_category {
            WeaponRange::Melee => Ability::score_to_mod(self.ability(&Ability::Str)),
            WeaponRange::Ranged => Ability::score_to_mod(self.ability(&Ability::Dex)),
        };
        ability_mod
        + // Proficiency bonus
        if self.proficient_with_weapon(weapon) { self.proficiency_bonus() } else { 0 }
    }
    /// Removes the ability of the character to speak the specified language
    pub fn unlearn_language(&mut self, language : &str) {
        self.languages.remove(language);
    }
    pub fn proficiency_bonus(&self) -> Modifier { 2 }
    /// Returns the current speed of the character, or throws an error if they have no race determining their base speed
    pub fn speed(&self) -> Result<&Speed, String> {
        match self.data.get_race(&self.race) {
            Some(r) => Ok(&r.speed),
            None => Err("Character has no race or race was not found.".to_owned())
        }
    }
    /// Returns true if the character speaks the specified language
    pub fn speaks(&self, language : &str) -> bool {
        self.languages.contains(language)
    }
    /// Returns whether or not the character can wear the specified armor
    pub fn can_equip(&self, armor : &Armor) -> bool {
        self.combat_proficiencies.contains(&CombatProficiency::ArmorCategory(armor.category))
    }
    /// Returns the skill level of the character in the specified skill,
    /// taking into account proficiencies from themself as well as from their race
    pub fn skill_level(&self, skill : &Skill) -> &SkillLevel {
        let own_skill_level = match self.skills.get(skill) {
            Some(prof) => prof,
            None => &SkillLevel::None,
        };
        if *own_skill_level == SkillLevel::None {
            match self.data.get_race(&self.race) {
                Some(r) => if r.skill_proficiencies.contains(skill) { &SkillLevel::Proficient } else { &SkillLevel::None },
                None => &SkillLevel::None,
            }            
        } else {
            own_skill_level
        }
    }
    /// Sets the skill level of a character in a skill independently of race or class
    pub fn set_skill_level(&mut self, skill : &Skill, level : SkillLevel) {
        self.skills.insert(skill.clone(), level);
    }
    /// Sets the ability score of the character to the specified score
    pub fn set_ability(&mut self, ability : &Ability, score : AbilityScore) {
        self.abilities.set(&ability, score);
    }
    /// Sets the race of the character to the specified score and removes all bonuses of their old race
    pub fn set_race(&mut self, race : &Race) -> Result<(),String> {
        self.unset_race()?;
        for (attr, bonus) in race.ability_bonuses.iter() {
            self.set_ability(attr, *self.abilities.get(attr) + *bonus);
        };
        for lang in race.languages.iter() {
            self.learn_language(lang.to_owned());
        }
        self.race = race.name.to_owned();
        Ok(())
    }
    /// Undo the effects of the current race
    fn unset_race(&mut self) -> Result<(), String> {
        if self.race != "" {
            let old_race = match self.data.get_race(&self.race) {
                Some(r) => r,
                None => { return Err("Old race not found!".to_owned()) }
            };
            for (attr, bonus) in old_race.ability_bonuses.iter() {
                self.set_ability(attr, *self.abilities.get(attr) - *bonus);
            } 
            for lang in old_race.languages.iter() {
                self.unlearn_language(lang);
            }
        }
        self.race = "".to_owned();
        Ok(())
    }
    /// Determine whether or not character is proficient with the specified weapon
    fn proficient_with_weapon(&self, weapon : &Weapon) -> bool {
        self.combat_proficiencies.contains(&CombatProficiency::WeaponCategory(weapon.category)) ||
        self.combat_proficiencies.contains(&CombatProficiency::Weapon(weapon.name.clone()))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Ability { Str, Dex, Con, Wis, Int, Cha }
pub type AbilityScore = i8; //Not unsigned because otherwise mismatching types make computing the ability modifier hell
type Modifier = i8;

struct Abilities {
    ability_values : HashMap<Ability, AbilityScore>,
}

use std::iter::{ FromIterator, repeat };

impl Abilities {
    fn new() -> Self {
        Abilities {
            ability_values : HashMap::from_iter(
                Iterator::zip(
                    vec![Ability::Str, Ability::Dex, Ability::Con, Ability::Wis, Ability::Int, Ability::Cha].iter().cloned(),
                    repeat(10).take(6)
                )
            )       
        }
    }
    /// This function can safely not deliver an &Option<AbilityValue> because
    /// the initialization in Abilities::new guarantees that every Ability always has
    /// an entry in the HashMap self.ability_values
    fn get(&self, ability : &Ability) -> &AbilityScore {
        self.ability_values.get(ability).unwrap()
    }
    fn set(&mut self, ability : &Ability, value : AbilityScore) {
        *self.ability_values.get_mut(ability).unwrap() = value;
    }
}

impl Ability {
    /// Returns the ability modifier corresponding to the ability score
    pub fn score_to_mod(ability_score : &AbilityScore) -> Modifier {
        let double_mod = ability_score - 10;
        // Since Rust rounds towards zero, we need to roll our own floor function here
        if double_mod > 0 { double_mod / 2 } else { ( double_mod - 1 ) / 2 }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Size { Tiny, Small, Medium, Large, Huge, Gargantuan }
pub type Speed = u16; //Speeds larger than 255 are theoretically possible, so no u8 here

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

#[cfg(test)]
mod test_character;