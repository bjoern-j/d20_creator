use crate::datastore::{ 
    Datastore, 
    Weapon, WeaponRange, Armor, 
    Race, Subrace, 
    Class, 
    Spell, 
    SpellLevel, SpellSlots,
    Skill, SkillLevel, CombatProficiency,
    Feat, FeatEffect, FeatPrerequisite,
};
use std::collections::{ HashMap, HashSet };

pub struct Character<'d> {
    pub name : String,
    data : &'d Datastore,
    level : Level,
    abilities : Abilities,
    race : String,
    subrace : String,
    class : String,
    languages : HashSet<String>,
    feats : HashSet<String>,
    skills : HashMap<Skill, SkillLevel>,
    combat_proficiencies : HashSet<CombatProficiency>,
    known_spells : KnownSpells,
}

#[derive(PartialEq, Eq, Hash)]
pub struct KnownSpell {
    name : String,
    casting_ability : Ability,
}
pub type KnownSpells = Vec<KnownSpell>;

impl KnownSpell {
    pub fn name(&self) -> &str { &self.name }
    pub fn ability(&self) -> &Ability { &self.casting_ability }
}

impl<'d> Character<'d> {
    /// Creates a new character that will draw on the data in the store passed to it here.
    /// This means characters cannot outlive the existence of the data store, which is reasonable
    /// since they need the data in it to know what e.g. feats do.
    pub fn new(data : &'d Datastore) -> Self {
        Character {
            name : String::new(),
            data : data,
            level : 1,
            abilities : Abilities::new(),
            race : String::new(),
            subrace : String::new(),
            class : String::new(),
            languages : HashSet::new(),
            feats : HashSet::new(),
            skills : HashMap::new(),
            combat_proficiencies : HashSet::new(),
            known_spells : Vec::new(),
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
    pub fn hit_die(&self) -> Option<&Die> {
        match self.data.get_class(&self.class) {
            Some(class) => Some(&class.hit_die),
            None => None,
        }
    }
    pub fn saving_throw(&self, ability : &Ability) -> Modifier {
        Ability::score_to_mod(self.ability(ability))
        +
        match self.data.get_class(&self.class) {
            Some(class) => if class.saving_throws.contains(ability) { self.proficiency_bonus() } else { 0 },
            None => 0,
        }
    }
    /// Endows the character with the ability to speak the specified language
    pub fn learn_language(&mut self, language : String) {
        self.languages.insert(language);
    }
    pub fn learn_spell(&mut self, spell : &Spell, ability : Ability) {
        self.known_spells.push(
            KnownSpell { name : spell.name.clone(), casting_ability : ability }
        );
    }
    pub fn unlearn_spell(&mut self, spell : &Spell, ability : Ability) {
        self.known_spells.retain( |known_spell| known_spell !=
            &KnownSpell { name : spell.name.clone(), casting_ability : ability }
        );
    }
    pub fn spells(&self) -> &KnownSpells {
        &self.known_spells
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
    pub fn skill_mod(&self, ability : &Ability, skill : &Skill) -> Modifier {
        Ability::score_to_mod(self.ability(ability))
        +
        match self.skill_level(skill) {
            SkillLevel::None => 0,
            SkillLevel::Proficient => self.proficiency_bonus(),
            SkillLevel::Expert => 2 * self.proficiency_bonus(),
        }
    }
    pub fn set_level(&mut self, level : Level) {
        self.level = level;
    }
    pub fn learn_feat(&mut self, feat : &Feat) -> Result<(), String> {
        if self.meets_prerequisites(feat) {
            self.feats.insert(feat.name.clone());
            for effect in &feat.effects {
                match effect {
                    FeatEffect::AbilityIncrease(ability, increase) => { 
                        *self.abilities.get_mut(&ability) = *self.ability(&ability) + increase; 
                    },
                    FeatEffect::SkillProficiency(_) => (),
                }    
            };
            Ok(())
        } else {
            Err("This character does not meet the prerequisites for this feat.".to_owned())
        }
    }
    fn meets_prerequisites(&self, feat : &Feat) -> bool {
        let mut result = true;
        for prereq in &feat.prerequisites {
            result = result && match prereq {
                FeatPrerequisite::MinimumAbility(ability, minimum) => *self.ability(&ability) >= *minimum,
            }
        }
        result
    }
    pub fn unlearn_feat(&mut self, feat : &Feat) {
        if self.feats.remove(&feat.name) {
            for effect in &feat.effects {
                match effect {
                    FeatEffect::AbilityIncrease(ability, increase) => { 
                        *self.abilities.get_mut(&ability) = *self.ability(&ability) - increase; 
                    },
                    FeatEffect::SkillProficiency(_) => (),
                }    
            };        
        }
    }
    /// Removes the ability of the character to speak the specified language
    pub fn unlearn_language(&mut self, language : &str) {
        self.languages.remove(language);
    }
    pub fn proficiency_bonus(&self) -> Modifier { 
        2 + ( (self.level - 1) / 4 )
    }
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
            let race_skill_level = match self.data.get_race(&self.race) {
                Some(r) =>  if r.skill_proficiencies.contains(skill) { 
                                &SkillLevel::Proficient 
                            } else { match r.get_subrace(&self.subrace) {
                                Some(sr) => if sr.skill_proficiencies.contains(skill) { &SkillLevel::Proficient } else { &SkillLevel::None },
                                None => &SkillLevel::None,
                            } },
                None => &SkillLevel::None,
            };
            if *race_skill_level == SkillLevel::None {
                let class_skill_level = match self.data.get_class(&self.class) {
                    Some(class) => if class.skill_proficiencies.contains(skill) { &SkillLevel::Proficient } else { &SkillLevel::None },
                    None => &SkillLevel::None,
                };
                if *class_skill_level == SkillLevel::None {
                    let mut feat_skill_level = &SkillLevel::None;
                    for feat in &self.feats {
                        let feat_data = self.data.get_feat(feat).unwrap();
                        if feat_data.effects.contains(&FeatEffect::SkillProficiency(skill.clone())) {
                            feat_skill_level = &SkillLevel::Proficient;
                            break;
                        } else { };
                    }
                    feat_skill_level
                } else {
                    class_skill_level
                }
            } else {
                race_skill_level
            }         
        } else {
            own_skill_level
        }
    }
    pub fn spell_slots(&self) -> SpellSlots {
        let no_slots = SpellLevel::slots(0,0,0,0,0,0,0,0,0);
        match self.data.get_class(&self.class) {
            Some(class) => SpellLevel::slots_for_level(&self.level, &class.spell_caster),
            None => no_slots,
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
    pub fn set_subrace(&mut self, subrace : &Subrace) -> Result<(), String> {
        self.unset_subrace()?;
        for (attr, bonus) in subrace.ability_bonuses.iter() {
            self.set_ability(attr, *self.abilities.get(attr) + *bonus);
        };
        for lang in subrace.languages.iter() {
            self.learn_language(lang.to_owned());
        };
        self.subrace = subrace.name.to_owned();
        Ok(())        
    }
    pub fn set_class(&mut self, class : &Class) {
        self.class = class.name.to_owned();
    }
    fn unset_subrace(&mut self) -> Result<(), String> {
        if self.subrace != "" {
            let old_subrace = match self.data.get_race(&self.race).unwrap().get_subrace(&self.subrace) {
                Some(r) => r,
                None => { return Err("Subrace not found!".to_owned()); },
            };
            for (attr, bonus) in old_subrace.ability_bonuses.iter() {
                self.set_ability(attr, *self.abilities.get(attr) - *bonus);
            };
            for lang in old_subrace.languages.iter() {
                self.unlearn_language(lang);
            };
        }
        self.subrace = "".to_owned();
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
        // First check whether the character itself is proficient
        self.combat_proficiencies.contains(&CombatProficiency::WeaponCategory(weapon.category)) ||
        self.combat_proficiencies.contains(&CombatProficiency::Weapon(weapon.name.clone())) ||
        // Then check whether the race might give them proficiency
        match self.data.get_race(&self.race) {
            Some(race) => { race.combat_proficiencies.contains(&CombatProficiency::WeaponCategory(weapon.category)) ||
                            race.combat_proficiencies.contains(&CombatProficiency::Weapon(weapon.name.clone())) ||
                            match race.get_subrace(&self.subrace) {
                                Some(subrace) => subrace.combat_proficiencies.contains(&CombatProficiency::WeaponCategory(weapon.category)) ||
                                                 subrace.combat_proficiencies.contains(&CombatProficiency::Weapon(weapon.name.clone())),
                                None => false,
                            } },
            None => false,
        } ||
        match self.data.get_class(&self.class) {
            Some(class) => { class.combat_proficiencies.contains(&CombatProficiency::WeaponCategory(weapon.category)) ||
                             class.combat_proficiencies.contains(&CombatProficiency::Weapon(weapon.name.clone())) },
            None => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Ability { Str, Dex, Con, Wis, Int, Cha }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Die { D4, D6, D8, D10, D12, D20 }
pub type AbilityScore = i8; //Not unsigned because otherwise mismatching types make computing the ability modifier hell
type Modifier = i8;
type Level = i8;

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
    fn get_mut(&mut self, ability : &Ability) -> &mut AbilityScore {
        self.ability_values.get_mut(ability).unwrap()
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

#[cfg(test)]
mod test_character;