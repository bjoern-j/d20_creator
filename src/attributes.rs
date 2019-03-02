pub type AttributeValue = i8;

#[derive(PartialEq,Eq,Hash)]
#[derive(Copy,Clone)]
pub enum AttributeName {
    Str,
    Dex,
    Con,
    Wis,
    Int,
    Cha,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub struct Attributes {
    str : AttributeValue,
    dex : AttributeValue,
    con : AttributeValue,
    wis : AttributeValue,
    int : AttributeValue,
    cha : AttributeValue,
}

impl Attributes {
    /// Returns the average stats array with all zero modifiers
    pub fn default() -> Self { Attributes{ str : 10, dex : 10, con : 10, wis : 10, int : 10, cha : 10 } }
    /// Creates a new attribute array with the specified values
    pub fn new(str : AttributeValue, dex : AttributeValue, con : AttributeValue, wis : AttributeValue, int : AttributeValue, cha : AttributeValue ) -> Self {
        Attributes{ str:str, dex:dex, con:con, wis:wis, int:int, cha:cha }
    }
    /// Returns the modifiers for these attributes
    pub fn get_modifiers(&self) -> Self {
        Attributes { 
            str : Self::compute_mod(self.str),
            dex : Self::compute_mod(self.dex),
            con : Self::compute_mod(self.con),
            wis : Self::compute_mod(self.wis),
            int : Self::compute_mod(self.int),
            cha : Self::compute_mod(self.cha), }
    }
    /// Get the value for the specified attribute
    pub fn get(&self, attr : AttributeName) -> AttributeValue {
        match attr {
            AttributeName::Str => self.str,
            AttributeName::Dex => self.dex,
            AttributeName::Con => self.con,
            AttributeName::Int => self.int, 
            AttributeName::Wis => self.wis,
            AttributeName::Cha => self.cha,
        }
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