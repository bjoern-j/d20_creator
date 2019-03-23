use super::*;
use std::iter::FromIterator;
use std::collections::HashSet;
use feats::{MechanicalFeat, NonMechanicalFeat};
use spells::{SpellLevel, SpellSchool, SpellComponent};
#[test]
fn test_name() {
    let mut builder = Builder::new();
    builder.set_name("John".to_owned());
    assert_eq!(builder.character().name(), "John");
}
#[test]
fn test_attributes() {
    let mut builder = Builder::new();
    builder.set_attribute(Attribute::Str, 15);
    assert_eq!(builder.character().attribute(Attribute::Str), 15);
    assert_eq!(builder.character().attribute(Attribute::Dex), 10);
}
#[test]
fn test_race() {
    let mut builder = get_elf_dwarf_builder();
    builder.set_race("Elf");
    assert_eq!(builder.character().attribute(Attribute::Dex), 12);
    assert_eq!(builder.character().attribute(Attribute::Int), 11);
}
#[test]
fn test_unset_race() {
    let mut builder = get_elf_dwarf_builder();
    builder.set_race("Elf");
    builder.set_race("Dwarf");
    assert_eq!(builder.character().attribute(Attribute::Str), 12);
    assert_eq!(builder.character().attribute(Attribute::Dex), 10);
    assert_eq!(builder.character().attribute(Attribute::Int), 9);
}
#[test]
fn test_size() {
    let mut builder = get_elf_dwarf_builder();
    builder.set_race("Elf");
    assert_eq!(builder.character().size(), Size::Medium);
    builder.set_race("Dwarf");
    assert_eq!(builder.character().size(), Size::Small);
}
#[test]
fn test_speed() {
    let mut builder = get_elf_dwarf_builder();
    builder.set_race("Elf");
    assert_eq!(builder.character().speed(), 35);
    builder.set_race("Dwarf");
    assert_eq!(builder.character().speed(), 30);
}
#[test]
fn test_language() {
    let mut builder = Builder::new();
    builder.add_language( Language{ 
        name : "Common".to_owned(), 
        long_text : "The language of the common man.".to_owned() }
    );
    builder.add_character_language("Common");
    assert_eq!(builder.character().speaks("Common"),true);
}
#[test]
fn test_simple_skills() {
    let mut builder = Builder::new();
    builder.set_skill_level(&Skill::Acrobatics, SkillLevel::Proficient);
    assert_eq!(builder.character().skill_level(&Skill::Athletics), SkillLevel::None);
    assert_eq!(builder.character().skill_level(&Skill::Acrobatics), SkillLevel::Proficient);
}
#[test]
fn test_parametrized_skills() {
    let mut builder = Builder::new();
    let lute = Skill::MusicalInstrument("Lute".to_owned());
    assert_eq!(builder.character().skill_level(&lute), SkillLevel::None);
    builder.set_skill_level(&lute, SkillLevel::Expert);
}
#[test]
fn test_non_mechanical_feat() {
    let mut builder = Builder::new();
    let darkvision = NonMechanicalFeat{
        name : "Darkvision".to_owned(),
        long_text : "The character can see in total darkness".to_owned(),
    };
    builder.add_feat( Rc::new(darkvision) );
    assert_eq!(builder.character().has_feat("Darkvision"), false);
    builder.add_feat_to_character("Darkvision");
    assert_eq!(builder.character().has_feat("Darkvision"), true);
}
#[test]
fn test_mechanical_feat() {
    let mut builder = get_strong_builder();
    builder.add_feat_to_character("Strong");
    assert_eq!(builder.character().has_feat("Strong"), true);
    assert_eq!(builder.character().attribute(Attribute::Str), 12);
}
#[test]
fn test_undo_mechanical_feat() {
    let mut builder = get_strong_builder();
    builder.add_feat_to_character("Strong");
    builder.remove_feat_from_character("Strong");
    assert_eq!(builder.character().attribute(Attribute::Str), 10);
}
#[test]
fn test_race_with_feats() {
    let mut builder = get_orc_builder();
    builder.set_race("Orc");
    assert_eq!(builder.character().has_feat("Strong"), true);
    assert_eq!(builder.character().attribute(Attribute::Str), 12);
    builder.add_race(get_elf_race());
    builder.set_race("Elf");
    assert_eq!(builder.character().has_feat("Strong"), false);
    assert_eq!(builder.character().attribute(Attribute::Str), 10);
}
#[test]
fn test_weapon_category_proficiency() {
    let mut builder = Builder::new();
    let martial = &WeaponOrArmor::WeaponCategory(WeaponCategory::Martial);
    assert!(!builder.character().proficient_with(martial));
    builder.add_weapon_or_armor_proficiency_to_character(martial);
    assert!(builder.character().proficient_with(martial));
}
#[test]
fn test_armor_category_proficiency() {
    let mut builder = Builder::new();
    let shield = &WeaponOrArmor::ArmorCategory(ArmorCategory::Shield);
    assert!(!builder.character().proficient_with(shield));
    builder.add_weapon_or_armor_proficiency_to_character(shield);
    assert!(builder.character().proficient_with(shield));
}
#[test]
fn test_weapon_proficiency() {
    let mut builder = Builder::new();
    let longsword = Weapon::new("Longsword".to_owned());
    builder.add_weapon(longsword);
    let longsword_proficiency = &WeaponOrArmor::Weapon("Longsword".to_owned());
    builder.add_weapon_or_armor_proficiency_to_character(longsword_proficiency);
    assert!(builder.character().proficient_with(longsword_proficiency));
}
#[test]
fn test_subrace() {
    let mut builder = Builder::new();
    builder.add_race(get_halfbreeds_race());
    builder.set_race("Halfbreed");
    builder.set_subrace("Half-Elf");
    assert_eq!(builder.character().attribute(Attribute::Dex), 12);
    builder.set_subrace("Half-Orc");
    assert_eq!(builder.character().attribute(Attribute::Str), 12);
    assert_eq!(builder.character().attribute(Attribute::Dex), 10);
}
#[test]
fn test_subrace_is_unset_when_switching_race() {
    let mut builder = Builder::new();
    builder.add_race(get_halfbreeds_race());
    builder.add_race(get_elf_race());
    builder.set_race("Halfbreed");
    builder.set_subrace("Half-Orc");
    builder.set_race("Elf");
    assert_eq!(builder.character().attribute(Attribute::Str), 10);
    assert_eq!(builder.character().attribute(Attribute::Con), 10);
}
#[test]
fn test_spell() {
    let mut builder = Builder::new();
    builder.add_spell(
        Spell {
            name : "Magic Missile".to_owned(),
            level : SpellLevel::First,
            school : SpellSchool::Evocation,
            casting_time : "1 action".to_owned(),
            components : HashSet::from_iter(
                vec![SpellComponent::Verbal, SpellComponent::Somatic].iter().cloned()
            ),
            duration : "Instantaneous".to_owned(),
            long_text : "Hits an enemy for 1d4+1 damage".to_owned(),
        }
    );
    builder.learn_spell("Magic Missile", Attribute::Int);
    assert!(builder.character.knows_spell("Magic Missile"));
    assert_eq!(builder.character.spellcasting_ability("Magic Missile"), Attribute::Int);
}
#[test]
fn test_class() {
    let mut builder = Builder::new();
    builder.add_class(get_commoner_class());
    builder.set_class("Commoner");
    assert_eq!(builder.character().hit_die(), Die::D4);
    assert!(builder.character().proficient_with(&WeaponOrArmor::ArmorCategory(ArmorCategory::Light)));
    assert_eq!(builder.character().skill_level(&Skill::Perception), SkillLevel::Proficient);
}

#[test]
fn test_saving_throws() {
    let mut builder = Builder::new();
    builder.add_class(get_commoner_class());
    builder.set_class("Commoner");
    assert_eq!(builder.character().saving_throw_mod(&Attribute::Str), 0);
    assert_eq!(builder.character().saving_throw_mod(&Attribute::Int), 2);
}

fn get_commoner_class() -> Class {
    Class {
        name : "Commoner".to_owned(),
        hit_die : Die::D4,
        weapon_and_armor_proficiencies : vec![
            WeaponOrArmor::WeaponCategory(WeaponCategory::Simple), 
            WeaponOrArmor::ArmorCategory(ArmorCategory::Light)
        ],
        skill_proficiencies : vec![
            Skill::Perception,
            Skill::Vehicle("Cart".to_owned())
        ],
        saving_throws : vec![
            Attribute::Int,
            Attribute::Cha
        ]
    }
}

fn get_elf_dwarf_builder() -> Builder {
    let mut builder = Builder::new();
    builder.add_race(get_elf_race());
    builder.add_race(get_dwarf_race());
    builder
}

fn get_orc_builder() -> Builder {
    let mut builder = get_strong_builder();
    builder.add_race(get_orc_race());
    builder
}

fn get_strong_builder() -> Builder {
    let mut builder = Builder::new();
    let strong = MechanicalFeat{
        name : "Strong".to_owned(),
        long_text : "This character is really strong".to_owned(),
        effect : Box::new(|ch : &mut Character| { ch.attributes.insert(Attribute::Str, ch.attributes.get(&Attribute::Str).unwrap() + 2); }),
        reverse_effect : Box::new(|ch : &mut Character| { ch.attributes.insert(Attribute::Str, ch.attributes.get(&Attribute::Str).unwrap() - 2); })
    };
    builder.add_feat(Rc::new(strong));
    builder
}

fn get_halfbreeds_race() -> Race {
    let mut halfbreed = Race::new(
        "Halfbreed".to_owned(),
        HashMap::from_iter(
            [(Attribute::Con, 1)].iter().cloned()
        ),
        Size::Medium,
        30,
        Vec::new(),
    );
    halfbreed.add_subrace(
        "Half-Elf".to_owned(),
        HashMap::from_iter(
            [(Attribute::Dex, 2)].iter().cloned()
        ),
        Vec::new(),
    );
    halfbreed.add_subrace(
        "Half-Orc".to_owned(),
        HashMap::from_iter(
            [(Attribute::Str, 2)].iter().cloned()
        ),
        Vec::new(),
    );
    halfbreed
}

fn get_elf_race() -> Race {
    Race::new(
        "Elf".to_owned(),
        HashMap::from_iter(
            [(Attribute::Dex, 2), (Attribute::Int, 1)].iter().cloned()
        ),
        Size::Medium,
        35,
        Vec::new(),
    )
}

fn get_dwarf_race() -> Race {
    Race::new(
        "Dwarf".to_owned(),
        HashMap::from_iter(
            [(Attribute::Str, 2), (Attribute::Int, -1)].iter().cloned()
        ),
        Size::Small,
        30,
        Vec::new(),
    )
}

fn get_orc_race() -> Race {
    Race::new(
        "Orc".to_owned(),
        HashMap::new(),
        Size::Medium,
        30,
        vec!["Strong".to_owned()],
    )
}