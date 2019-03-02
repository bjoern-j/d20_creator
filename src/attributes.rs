use std::collections::{HashMap};
use std::iter::FromIterator;

pub type AttributeValue = i8;
pub type AttributeBonus = (AttributeName, AttributeValue);

#[derive(PartialEq,Eq,Hash)]
#[derive(Copy,Clone)]
#[derive(Debug)]
pub enum AttributeName {
    Str,
    Dex,
    Con,
    Wis,
    Int,
    Cha,
}

pub type AttributeArray = HashMap<AttributeName, AttributeValue>;

#[derive(Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub struct Attributes {
    attributes : AttributeArray,
}

impl Attributes {
    /// Returns the average stats array with all zero modifiers
    pub fn default() -> Self { 
        Attributes::new(10,10,10,10,10,10)
    }
    /// Creates a new attribute array with the specified values
    pub fn new(str : AttributeValue, dex : AttributeValue, con : AttributeValue, wis : AttributeValue, int : AttributeValue, cha : AttributeValue ) -> Self {
        Attributes{ attributes : HashMap::from_iter(
            [(AttributeName::Str, str), 
             (AttributeName::Dex, dex), 
             (AttributeName::Con, con), 
             (AttributeName::Wis, wis),
             (AttributeName::Int, int), 
             (AttributeName::Cha, cha)].iter().cloned())  }
    }
    /// Returns the modifiers for these attributes
    pub fn get_modifiers(&self) -> Self {
        let mut modified_attrs = HashMap::new();
        for (name, value) in self.attributes.iter() {
            modified_attrs.insert(*name, Self::compute_mod(*value));
        }
        Attributes{ attributes : modified_attrs }
    }
    /// Returns the value for the specified attribute
    pub fn get(&self, attr : AttributeName) -> AttributeValue {
        *self.attributes.get(&attr).unwrap()
    }
    pub fn set(&mut self, attr : AttributeName, value : AttributeValue) {
        self.attributes.insert(attr,value);
    }
    /// Applies the specified bonuses to itself and returns the modified attribute array
    pub fn apply_bonuses(&self, bonuses : &AttributeArray) -> Attributes {
        let mut modified_attrs = HashMap::new();
        for (name, value) in self.attributes.iter() {
            match bonuses.get(name) {
                Some(bonus) => modified_attrs.insert(*name, value + bonus),
                None => modified_attrs.insert(*name, *value),
            };
        }
        Attributes{ attributes : modified_attrs }
    }
    /// Computes the modifier for a given attribute value. 
    /// Not a one-line "(value - 10)/2" because Rust rounds towards zero, 
    /// but we want flooring also for negative values.
    fn compute_mod(value : AttributeValue) -> AttributeValue { 
        let double_mod = value - 10;
        if double_mod > 0 {
            double_mod / 2
        } else {
            (double_mod - 1) / 2
        }
    }
}