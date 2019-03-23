use std::collections::HashSet;

pub struct Spell {
    pub name : String,
    pub level : SpellLevel,
    pub school : SpellSchool,
    pub casting_time : String,
    pub components : HashSet<SpellComponent>,
    pub duration : String,
    pub long_text : String,
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
pub enum SpellLevel { Cantrip, First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth, Ninth }

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
pub enum SpellSchool {
    Abjuration,
    Conjuration, 
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub enum SpellComponent {
    Verbal,
    Somatic,
    MaterialPresent(String),
    MaterialConsumed(String),
}

impl Spell {
    pub fn name(&self) -> &str { &self.name }
}