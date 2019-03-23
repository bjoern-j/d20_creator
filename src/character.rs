use crate::datastore::Datastore;
use std::collections::HashMap;

pub struct Character<'d> {
    pub name : String,
    data : &'d Datastore,
    abilities : Abilities,
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
        }
    }
    /// Returns the current ability score of the character for the ability
    pub fn ability(&self, ability : &Ability) -> &AbilityScore {
        self.abilities.get(ability)
    }
    pub fn set_ability(&mut self, ability : &Ability, score : AbilityScore) {
        self.abilities.set(&ability, score);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Ability { Str, Dex, Con, Wis, Int, Cha }
type AbilityScore = i8; //Not unsigned because otherwise mismatching types make computing the ability modifier hell
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

mod test_character;