use std::collections::{ HashMap, HashSet };
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpellCaster { None, Third, Half, Full }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpellLevel { Cantrip, First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth, Ninth }
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum SpellComponent { Verbal, Somatic, Material(String) }
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpellSchool { Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation }
pub type SpellSlots = HashMap<SpellLevel, u8>;

pub struct Spell {
    pub name : String,
    pub long_text : String,
    pub level : SpellLevel,
    pub school : SpellSchool,
    pub casting_time : String,
    pub components : HashSet<SpellComponent>,
    pub duration : String,
}

impl SpellLevel {
    pub fn slots(first : u8, second : u8, third : u8, fourth : u8, fifth : u8, sixth : u8, seventh : u8, eighth : u8, ninth : u8) -> SpellSlots {
        HashMap::from_iter(
            Iterator::zip(
                vec![SpellLevel::First, SpellLevel::Second, SpellLevel::Third, 
                     SpellLevel::Fourth, SpellLevel::Fifth, SpellLevel::Sixth, 
                     SpellLevel::Seventh, SpellLevel::Eighth, SpellLevel::Ninth].iter().cloned(),
                vec![first, second, third,
                     fourth, fifth, sixth,
                     seventh, eighth, ninth].iter().cloned()
            )
        )
    }
    pub fn slots_for_level(level : &i8, caster : &SpellCaster) -> SpellSlots {
        match caster {
            SpellCaster::None => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
            SpellCaster::Full => match level {
                1 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                6 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                8 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                9 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                10 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                11 => SpellLevel::slots(4,3,3,3,2,1,0,0,0),
                12 => SpellLevel::slots(4,3,3,3,2,1,0,0,0),
                13 => SpellLevel::slots(4,3,3,3,2,1,1,0,0),
                14 => SpellLevel::slots(4,3,3,3,2,1,1,0,0),
                15 => SpellLevel::slots(4,3,3,3,2,1,1,1,0),
                16 => SpellLevel::slots(4,3,3,3,2,1,1,1,0),
                17 => SpellLevel::slots(4,3,3,3,2,1,1,1,1),
                18 => SpellLevel::slots(4,3,3,3,3,1,1,1,1),
                19 => SpellLevel::slots(4,3,3,3,3,2,1,1,1),
                20 => SpellLevel::slots(4,3,3,3,3,2,2,1,1),
                _ => panic!("Invalid level")
            },
            SpellCaster::Half => match level {
                1 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                6 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                8 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                9 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                10 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                11 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                12 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                13 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                14 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                15 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                16 => SpellLevel::slots(4,3,3,2,0,0,0,0,0),
                17 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                18 => SpellLevel::slots(4,3,3,3,1,0,0,0,0),
                19 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                20 => SpellLevel::slots(4,3,3,3,2,0,0,0,0),
                _ => panic!("Invalid level")
            },
            SpellCaster::Third => match level {
                1 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                2 => SpellLevel::slots(0,0,0,0,0,0,0,0,0),
                3 => SpellLevel::slots(2,0,0,0,0,0,0,0,0),
                4 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                5 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                6 => SpellLevel::slots(3,0,0,0,0,0,0,0,0),
                7 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                8 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                9 => SpellLevel::slots(4,2,0,0,0,0,0,0,0),
                10 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                11 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                12 => SpellLevel::slots(4,3,0,0,0,0,0,0,0),
                13 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                14 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                15 => SpellLevel::slots(4,3,2,0,0,0,0,0,0),
                16 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                17 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                18 => SpellLevel::slots(4,3,3,0,0,0,0,0,0),
                19 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                20 => SpellLevel::slots(4,3,3,1,0,0,0,0,0),
                _ => panic!("Invalid level")
            },
        }
    }
}