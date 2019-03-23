use std::collections::HashMap;
use std::rc::Rc;

mod character;
mod race;
mod size;
mod language;
mod skill;
mod feats;
mod weapons;
mod armor;
mod spells;
mod classes;

use size::Size;
use race::Race;
use language::Language;
use character::{Character, WeaponOrArmor};
use character::attributes::Attribute;
use skill::{Skill, SkillLevel};
use feats::Feat;
use weapons::{Weapon, WeaponCategory};
use armor::ArmorCategory;
use spells::{Spell};
use classes::Class;

type AttributeValue = character::attributes::Value;
type Speed = u16; //u8 is too small since speeds larger than 255 are theoretically possible
type Feats = HashMap<String, Rc<Feat>>;
type Modifier = i8; //Modifiers aren't ever large, but they are definitely signed

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Die { D4, D6, D8, D12, D20, }

pub struct Builder {
    character : Character,
    races : HashMap<String, Race>,
    languages : HashMap<String, Language>,
    feats : Feats,
    weapons : HashMap<String, Weapon>,
    spells : HashMap<String, Spell>,
    classes : HashMap<String, Class>,
}

impl Builder {
    pub fn new() -> Self {
        Builder{
            character : Character::new(),
            races : HashMap::new(),
            languages : HashMap::new(),
            feats : HashMap::new(),
            weapons : HashMap::new(),
            spells : HashMap::new(),
            classes : HashMap::new(),
        }
    }
    pub fn set_name(&mut self, name : String) {
        self.character.set_name(name);        
    }
    pub fn character(&self) -> &Character {
        &self.character
    }
    pub fn set_attribute(&mut self, attribute : Attribute, value : AttributeValue) {
        self.character.set_attribute(attribute, value);
    }
    pub fn add_weapon_or_armor_proficiency_to_character(&mut self, prof : &WeaponOrArmor) {
        self.character.add_weapon_or_armor_proficiency(prof);
    }
    pub fn add_race(&mut self, race : Race) {
        self.races.insert(race.name().to_owned(), race);
    }
    pub fn add_weapon(&mut self, weapon : Weapon) {
        self.weapons.insert(weapon.name().to_owned(), weapon);
    }
    pub fn add_spell(&mut self, spell : Spell) {
        self.spells.insert(spell.name().to_owned(), spell);
    }
    pub fn add_class(&mut self, class : Class) {
        self.classes.insert(class.name().to_owned(), class);
    }
    pub fn set_class(&mut self, class : &str) {
        let new_class = self.classes.get(class).unwrap();
        self.character.set_class(class.to_owned());
        self.character.set_hit_die(new_class.hit_die);
        for weapon_or_armor in new_class.weapon_and_armor_proficiencies.iter() {
            self.character.add_weapon_or_armor_proficiency(weapon_or_armor);
        }
        for skill in new_class.skill_proficiencies.iter() {
            self.character.set_skill_level(skill, SkillLevel::Proficient);
        }
        for attr in new_class.saving_throws.iter() {
            self.character.add_saving_throw_proficiency(*attr);
        }
    }
    pub fn set_race(&mut self, race : &str) {
        self.unset_race();
        self.character.set_race(race.to_owned());
        let new_race = self.races.get(race).unwrap();
        for (attr, val) in new_race.attributes.iter() {
            let char_attr = self.character.get_mut_attribute(attr);
            *char_attr += val;
        };
        for feat in new_race.feats.iter() {
            Self::feat_to_char(self.feats.get(feat).unwrap(), &mut self.character);
        }
        self.character.set_size(new_race.size);
        self.character.set_speed(new_race.speed);
    }
    pub fn set_subrace(&mut self, subrace : &str) {
        let character_race = match self.character.race() {
            Some(race_name) => race_name.to_owned(),
            None => panic!("Tried to set subrace before race!"),
        };
        let actual_race = self.races.get(&character_race).unwrap();
        Self::unset_subrace(&mut self.character, &self.feats, actual_race);
        let new_subrace = actual_race.subraces.get(subrace).unwrap();
        for (attr, val) in new_subrace.attributes.iter() {
            let char_attr = self.character.get_mut_attribute(attr);
            *char_attr += val;
        };
        for feat in new_subrace.feats.iter() {
            Self::feat_to_char(self.feats.get(feat).unwrap(), &mut self.character);
        };
        self.character.set_subrace(subrace.to_owned());
    }
    pub fn learn_spell(&mut self, spell : &str, spellcasting_ability : Attribute) {
        self.character.add_spell(spell.to_owned(), spellcasting_ability);
    }
    pub fn add_language(&mut self, language : Language) {
        self.languages.insert(language.name.clone(), language);
    }
    pub fn add_feat(&mut self, feat : Rc<Feat>) {
        self.feats.insert(feat.name().to_owned(), feat);
    }
    pub fn add_feat_to_character(&mut self, feat : &str) {
        Self::feat_to_char(self.feats.get(feat).unwrap(), &mut self.character);
    }
    pub fn remove_feat_from_character(&mut self, feat : &str) {
        Self::feat_from_char(self.feats.get(feat).unwrap(), &mut self.character);
    }
    pub fn add_character_language(&mut self, language : &str) {
        self.character.add_language(language.to_owned());
    }
    pub fn set_skill_level(&mut self, skill : &Skill, level : SkillLevel) {
        self.character.set_skill_level(skill, level);
    }
    fn feat_to_char(feat : &Rc<Feat>, ch : &mut Character) {
        ch.add_feat(feat.name().to_owned());
        feat.apply_to(ch);
    }
    fn feat_from_char(feat : &Rc<Feat>, ch : &mut Character) {
        feat.reverse_effect_on(ch);
        ch.remove_feat(feat.name());        
    }
    fn unset_subrace(ch : &mut Character, feats : &Feats, old_race : &Race) {
        match ch.subrace() {
            Some(subrace) => {
                let old_subrace = old_race.subraces.get(subrace).unwrap();
                for (attr, val) in old_subrace.attributes.iter() {
                    let char_attr = ch.get_mut_attribute(attr);
                    *char_attr -= val;
                };
                for feat in &old_subrace.feats {
                    Self::feat_from_char(feats.get(feat).unwrap(), ch);
                };                        
            },
            None => (),
        };
    }
    fn unset_race(&mut self) {
        match self.character.race() {
            Some(race) => {
                let old_race = self.races.get(race).unwrap();
                for (attr, val) in old_race.attributes.iter() {
                    let char_attr = self.character.get_mut_attribute(attr);
                    *char_attr -= val;
                };
                for feat in &old_race.feats {
                    Self::feat_from_char(self.feats.get(feat).unwrap(), &mut self.character);
                };
                Self::unset_subrace(&mut self.character, &self.feats, old_race);
                },
            None => (),
        }
        self.character.clear_race();
        self.character.clear_size();
    }
}

#[cfg(test)]
mod test_builder_1;