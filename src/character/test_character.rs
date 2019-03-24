use super::*;
use crate::datastore::Race;

#[cfg(test)]
mod test_non_data_dependent_features {
    use super::*;
    #[test]
    fn test_naming_the_character() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        ch.name = "Dude".to_owned();
    }
    #[test]
    fn test_character_abilities_are_initially_10() {
        let data = Datastore::new();
        let ch = Character::new(&data);
        assert_eq!(*ch.ability(&Ability::Str), 10);
        assert_eq!(Ability::score_to_mod(ch.ability(&Ability::Con)), 0);
    }
    #[test]
    fn test_setting_character_abilities() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        ch.set_ability(&Ability::Int, 13);
        assert_eq!(*ch.ability(&Ability::Int), 13);
        assert_eq!(Ability::score_to_mod(ch.ability(&Ability::Int)), 1);
    }
    #[test]
    fn test_learn_language() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        ch.learn_language("Gobbledidok".to_owned());
        assert!(ch.speaks("Gobbledidok"));
        ch.unlearn_language("Gobbledidok");
        assert!(!ch.speaks("Gobbledidok"));
    }
    #[test]
    fn test_learn_skills() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        assert_eq!(*ch.skill_level(&Skill::Perception), SkillLevel::None);
        ch.set_skill_level(&Skill::Perception, SkillLevel::Proficient);
        assert_eq!(*ch.skill_level(&Skill::Perception), SkillLevel::Proficient);
    }
    #[test]
    fn test_learn_parametrized_skills() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        ch.set_skill_level(&Skill::Vehicle("Car".to_owned()), SkillLevel::Expert);
        assert_eq!(*ch.skill_level(&Skill::Vehicle("Car".to_owned())), SkillLevel::Expert);
    }
}

#[cfg(test)]
mod test_race_data_dependent_features {
    use super::*;
    #[test]
    fn test_setting_race_on_the_character() {
        let data = datastore_with_test_races();
        let mut ch = Character::new(&data);
        let angel = data.get_race("Angel").unwrap();
        ch.set_race(angel).unwrap();
        assert_eq!(*ch.ability(&Ability::Wis), 12);
        assert_eq!(*ch.size().unwrap(), Size::Medium);
        assert_eq!(*ch.speed().unwrap(), 40);
        assert!(ch.speaks("Angelic"));
        assert!(!ch.speaks("Demonic"));
        assert_eq!(*ch.skill_level(&Skill::Persuasion), SkillLevel::Proficient);
    }
    #[test]
    fn test_setting_different_races_undoes_effects_of_first_race() {
        let data = datastore_with_test_races();
        let mut ch = Character::new(&data);
        let angel = data.get_race("Angel").unwrap();
        let demon = data.get_race("Demon").unwrap();
        ch.set_race(angel).unwrap();
        ch.set_race(demon).unwrap();
        assert_eq!(*ch.ability(&Ability::Con), 12);
        assert_eq!(*ch.ability(&Ability::Wis), 10);
        assert_eq!(*ch.size().unwrap(), Size::Large);
        assert_eq!(*ch.speed().unwrap(), 30);
        assert!(!ch.speaks("Angelic"));
        assert!(ch.speaks("Demonic"));
        assert_eq!(*ch.skill_level(&Skill::Persuasion), SkillLevel::None);
        assert_eq!(*ch.skill_level(&Skill::Intimidation), SkillLevel::Proficient);
    }
    fn datastore_with_test_races() -> Datastore {
        let mut data = Datastore::new();
        data = add_races(data);
        data
    }
}

#[cfg(test)]
mod test_equipment_data_dependent_features {
    use super::*;
    use crate::datastore::WeaponCategory;
    #[test]
    fn test_weapon_proficiency() {
        let data = data_store_with_equipment();
        let mut ch = Character::new(&data);
        ch.add_combat_proficiency(CombatProficiency::Weapon("Bloodsword".to_owned()));
        assert_eq!(ch.get_attack_mod(data.get_weapon("Bloodsword").unwrap()), 2);
    }
    #[test]
    fn test_weapon_category_proficiency() {
        let data = data_store_with_equipment();
        let mut ch = Character::new(&data);
        ch.add_combat_proficiency(CombatProficiency::WeaponCategory(WeaponCategory::Martial));
        assert_eq!(ch.get_attack_mod(data.get_weapon("Bloodsword").unwrap()), 2);
    }
    #[test]
    fn test_weapon_ability_modifier() {
        let data = data_store_with_equipment();
        let mut ch = Character::new(&data);
        ch.add_combat_proficiency(CombatProficiency::WeaponCategory(WeaponCategory::Simple));
        ch.set_ability(&Ability::Str, 16);
        assert_eq!(ch.get_attack_mod(data.get_weapon("Bloodsword").unwrap()), 3);
        assert_eq!(ch.get_attack_mod(data.get_weapon("Beau's Bow").unwrap()), 2);
    }
    #[test]
    fn test_armor_category_proficiency() {
        let data = data_store_with_equipment();
        let mut ch = Character::new(&data);
        let armor = data.get_armor("Power Armor").unwrap();
        assert!(!ch.can_equip(armor));
        ch.add_combat_proficiency(CombatProficiency::ArmorCategory(ArmorCategory::Heavy));
        assert!(ch.can_equip(armor));
    }

    fn data_store_with_equipment() -> Datastore {
        let mut data = Datastore::new();
        data = add_equipment(data);
        data
    }
}

fn add_races(data : Datastore) -> Datastore {
    let mut data = data;
    data.add_race(
        Race {
            name : "Angel".to_owned(),
            long_text : "A group of divine figures.".to_owned(),
            ability_bonuses : HashMap::from_iter(
                vec![(Ability::Wis, 2)].iter().cloned()
            ),
            size : Size::Medium,
            speed : 40,
            languages : vec!["Angelic".to_owned()],
            skill_proficiencies : vec![Skill::Persuasion],
        }
    );      
    data.add_race(
        Race {
            name : "Demon".to_owned(),
            long_text : "MWHAHAHAHAHAHAHA".to_owned(),
            ability_bonuses : HashMap::from_iter(
                vec![(Ability::Con, 2)].iter().cloned()
            ),
            size : Size::Large,
            speed : 30,
            languages : vec!["Demonic".to_owned()],
            skill_proficiencies : vec![Skill::Intimidation],
        }
    );    
    data
}

fn add_equipment(data : Datastore) -> Datastore {
    let mut data = data;
    data.add_weapon(
        Weapon {
            name : "Bloodsword".to_owned(),
            category : WeaponCategory::Martial,
            range_category : WeaponRange::Melee,
            reach : 5,
        }
    );
    data.add_weapon(
        Weapon {
            name : "Beau's Bow".to_owned(),
            category : WeaponCategory::Simple,
            range_category : WeaponRange::Ranged,
            reach : 60,
        }
    );
    data.add_armor(
        Armor {
            name : "Power Armor".to_owned(),
            category : ArmorCategory::Heavy,
        }
    );
    data
}