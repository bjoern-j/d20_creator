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

type AttributeValue = character::attributes::Value;
type Speed = u16; //u8 is too small since speeds larger than 255 are theoretically possible
type Feats = HashMap<String, Rc<Feat>>;

pub struct Builder {
    character : Character,
    races : HashMap<String, Race>,
    languages : HashMap<String, Language>,
    feats : Feats,
    weapons : HashMap<String, Weapon>,
    spells : HashMap<String, Spell>,
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
        }
    }
    pub fn set_name(&mut self, name : String) {
        self.character.name = Some(name);        
    }
    pub fn character(&self) -> &Character {
        &self.character
    }
    pub fn set_attribute(&mut self, attribute : Attribute, value : AttributeValue) {
        self.character.attributes.insert(attribute, value);
    }
    pub fn add_weapon_or_armor_proficiency_to_character(&mut self, prof : &WeaponOrArmor) {
        match prof {
            WeaponOrArmor::WeaponCategory(cat) => { self.character.weapon_category_proficiencies.insert(*cat); },
            WeaponOrArmor::ArmorCategory(cat) => { self.character.armor_proficiencies.insert(*cat); },
            WeaponOrArmor::Weapon(weapon) => { self.character.weapon_proficiencies.insert(weapon.to_owned()); }
        }
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
    pub fn set_race(&mut self, race : &str) {
        self.unset_race();
        self.character.race = Some(race.to_owned());
        let new_race = self.races.get(race).unwrap();
        for (attr, val) in new_race.attributes.iter() {
            let char_attr = self.character.attributes.get_mut(attr).unwrap();
            *char_attr += val;
        };
        for feat in new_race.feats.iter() {
            Self::feat_to_char(self.feats.get(feat).unwrap(), &mut self.character);
        }
        self.character.size = Some(new_race.size);
        self.character.speed = Some(new_race.speed);
    }
    pub fn set_subrace(&mut self, subrace : &str) {
        let character_race = match &self.character.race {
            Some(race_name) => race_name.to_owned(),
            None => panic!("Tried to set subrace before race!"),
        };
        let actual_race = self.races.get(&character_race).unwrap();
        Self::unset_subrace(&mut self.character, &self.feats, actual_race);
        let new_subrace = actual_race.subraces.get(subrace).unwrap();
        for (attr, val) in new_subrace.attributes.iter() {
            let char_attr = self.character.attributes.get_mut(attr).unwrap();
            *char_attr += val;
        };
        for feat in new_subrace.feats.iter() {
            Self::feat_to_char(self.feats.get(feat).unwrap(), &mut self.character);
        };
        self.character.subrace = Some(subrace.to_owned());
    }
    pub fn learn_spell(&mut self, spell : &str, spellcasting_ability : Attribute) {
        self.character.spells.insert(spell.to_owned(), spellcasting_ability);
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
        self.character.languages.insert(language.to_owned());
    }
    pub fn set_skill_level(&mut self, skill : Skill, level : SkillLevel) {
        self.character.skills.insert(skill, level);
    }
    fn feat_to_char(feat : &Rc<Feat>, ch : &mut Character) {
        ch.feats.insert(feat.name().to_owned());
        feat.apply_to(ch);
    }
    fn feat_from_char(feat : &Rc<Feat>, ch : &mut Character) {
        feat.reverse_effect_on(ch);
        ch.feats.remove(feat.name());        
    }
    fn unset_subrace(ch : &mut Character, feats : &Feats, old_race : &Race) {
        match &ch.subrace {
            Some(subrace) => {
                let old_subrace = old_race.subraces.get(subrace).unwrap();
                for (attr, val) in old_subrace.attributes.iter() {
                    let char_attr = ch.attributes.get_mut(attr).unwrap();
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
        match &self.character.race {
            Some(race) => {
                let old_race = self.races.get(race).unwrap();
                for (attr, val) in old_race.attributes.iter() {
                    let char_attr = self.character.attributes.get_mut(attr).unwrap();
                    *char_attr -= val;
                };
                for feat in &old_race.feats {
                    Self::feat_from_char(self.feats.get(feat).unwrap(), &mut self.character);
                };
                Self::unset_subrace(&mut self.character, &self.feats, old_race);
                },
            None => (),
        }
        self.character.race = None;
        self.character.size = None;
    }
}

#[cfg(test)]
mod test_builder_1;