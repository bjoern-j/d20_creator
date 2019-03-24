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
    #[test]
    fn test_skill_modifiers() {
        let data = Datastore::new();
        let mut ch = Character::new(&data);
        ch.set_skill_level(&Skill::Perception, SkillLevel::Proficient);
        ch.set_ability(&Ability::Wis, 14);
        assert_eq!(ch.skill_mod(&Ability::Wis, &Skill::Perception), 4);
        ch.set_level(10);
        assert_eq!(ch.skill_mod(&Ability::Wis, &Skill::Perception), 6)
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

#[cfg(test)]
mod test_race_subrace_and_equipment_data_dependent_features {
    use super::*;
    #[test]
    fn test_race_combat_proficiencies() {
        let data = data_store_with_races_subraces_and_equipment();
        let mut ch = Character::new(&data);
        let bow = data.get_weapon("Beau's Bow").unwrap();
        ch.set_race(data.get_race("Angel").unwrap()).unwrap();
        assert_eq!(ch.get_attack_mod(bow), 2);
        ch.set_race(data.get_race("Demon").unwrap()).unwrap();
        assert_eq!(ch.get_attack_mod(bow), 0);
        let sword = data.get_weapon("Bloodsword").unwrap();
        assert_eq!(ch.get_attack_mod(sword), 2);
    }
    #[test]
    fn test_subrace_combat_proficiencies() {
        let data = data_store_with_races_subraces_and_equipment();
        let mut ch = Character::new(&data);
        let bow = data.get_weapon("Beau's Bow").unwrap();
        let halfbreed = data.get_race("Halfbreed").unwrap();
        ch.set_race(halfbreed).unwrap();
        let half_angel = halfbreed.get_subrace("Half-Angel").unwrap();
        ch.set_subrace(half_angel).unwrap();        
        assert_eq!(ch.get_attack_mod(bow), 2);
    }

    fn data_store_with_races_subraces_and_equipment() -> Datastore {
        let mut data = Datastore::new();
        data = add_equipment(add_race_with_subraces(add_races(data)));
        data
    }
}

#[cfg(test)]
mod test_subrace_dependent_features {
    use super::*;
    #[test]
    fn test_set_subrace() {
        let data = data_store_with_subrace();
        let mut ch = Character::new(&data);
        let halfbreed = data.get_race("Halfbreed").unwrap();
        ch.set_race(halfbreed).unwrap();
        let half_angel = halfbreed.get_subrace("Half-Angel").unwrap();
        ch.set_subrace(half_angel).unwrap();
        assert_eq!(*ch.ability(&Ability::Wis), 11);
        assert!(ch.speaks("Angelic"));
        assert_eq!(*ch.skill_level(&Skill::Persuasion), SkillLevel::Proficient);
    }

    fn data_store_with_subrace() -> Datastore {
        let mut data = Datastore::new();
        data = add_race_with_subraces(data);
        data
    }
}

#[cfg(test)]
mod test_class_dependent_features{
    use super::*;
    #[test]
    fn test_setting_character_class() {
        let data = data_store_with_classes();
        let mut ch = Character::new(&data);
        let warrior = data.get_class("Warrior").unwrap();
        ch.set_class(warrior);
        assert_eq!(*ch.hit_die().unwrap(), Die::D10);
        assert_eq!(ch.saving_throw(&Ability::Str), 2);
    }

    fn data_store_with_classes() -> Datastore {
        let mut data = Datastore::new();
        data = add_classes(data);
        data
    }
}

fn add_classes(data : Datastore) -> Datastore {
    let mut data = data;
    data.add_class(
        Class {
            name : "Warrior".to_owned(),
            long_text : "A brave fighter".to_owned(),
            hit_die : Die::D10,
            saving_throws : vec![Ability::Str, Ability::Con],
        }
    );
    data
}

fn add_race_with_subraces(data : Datastore) -> Datastore {
    let mut data = data;
    let mut halfbreed = Race {
        name : "Halfbreed".to_owned(),
        long_text : "Daughter of two worlds.".to_owned(),
        ability_bonuses : HashMap::from_iter(
            vec![(Ability::Cha, 2)].iter().cloned()
        ),
        size : Size::Medium,
        speed : 35,
        languages : vec!["Common".to_owned()],
        skill_proficiencies : Vec::new(),
        combat_proficiencies : Vec::new(),
        subraces : HashMap::new(),
    };
    halfbreed.add_subrace(
        Subrace {
            name : "Half-Angel".to_owned(),
            long_text : "Fallen from heaven".to_owned(),
            ability_bonuses : HashMap::from_iter(
                vec![(Ability::Wis, 1)].iter().cloned()
            ),
            languages : vec!["Angelic".to_owned()],
            skill_proficiencies : vec![Skill::Persuasion],
            combat_proficiencies : vec![CombatProficiency::WeaponCategory(WeaponCategory::Simple)],
        }
    );
    data.add_race(halfbreed);
    data
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
            combat_proficiencies : vec![CombatProficiency::Weapon("Beau's Bow".to_owned())],
            subraces : HashMap::new(),
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
            combat_proficiencies : vec![CombatProficiency::Weapon("Bloodsword".to_owned())],
            subraces : HashMap::new(),
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