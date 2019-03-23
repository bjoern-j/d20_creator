#[cfg(test)]
mod test_non_data_dependent_features {
    use super::super::*;
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
}

#[cfg(test)]
mod test_data_dependent_features {
    use super::super::*;
    use crate::datastore::Race;
    #[test]
    fn test_setting_race_on_the_character() {
        let data = datastore_with_test_races();
        let mut ch = Character::new(&data);
        ch.set_race("Angel").unwrap();
        assert_eq!(*ch.ability(&Ability::Wis), 12);
        assert_eq!(*ch.size().unwrap(), Size::Medium);
    }
    #[test]
    fn test_setting_different_races_undoes_effects_of_first_race() {
        let data = datastore_with_test_races();
        let mut ch = Character::new(&data);
        ch.set_race("Angel").unwrap();
        ch.set_race("Demon").unwrap();
        assert_eq!(*ch.ability(&Ability::Con), 12);
        assert_eq!(*ch.ability(&Ability::Wis), 10);
        assert_eq!(*ch.size().unwrap(), Size::Large);
    }
    fn datastore_with_test_races() -> Datastore {
        let mut data = Datastore::new();
        data.add_race(
            Race {
                name : "Angel".to_owned(),
                long_text : "A group of divine figures.".to_owned(),
                ability_bonuses : HashMap::from_iter(
                    vec![(Ability::Wis, 2)].iter().cloned()
                ),
                size : Size::Medium,
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
            }
        );
        data
    }
}