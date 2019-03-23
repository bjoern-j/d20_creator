use super::HashMap;
use std::vec::IntoIter;
use std::iter::{ FromIterator, repeat };

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Attribute{ Str, Dex, Con, Wis, Int, Cha, }

pub type Value = i8;
pub type Array = HashMap<Attribute, Value>;

use Attribute::*;

impl Attribute {
    pub fn into_iter() -> IntoIter<Attribute> {
        vec![Str, Dex, Con, Wis, Int, Cha].into_iter()
    }
    pub fn array_of_ten() -> Array {
        HashMap::from_iter(Iterator::zip(Attribute::into_iter(), repeat(10).take(6)))
    }
}