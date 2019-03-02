pub struct Character {
    name : String,
    pub attributes : Attributes,
}

type AttributeValue = i8;

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
    fn default() -> Self { Attributes{ str : 10, dex : 10, con : 10, wis : 10, int : 10, cha : 10 } }
    /// Returns the modifiers for these attributes
    fn get_modifiers(&self) -> Self {
        Attributes { 
            str : Self::compute_mod(self.str),
            dex : Self::compute_mod(self.dex),
            con : Self::compute_mod(self.con),
            wis : Self::compute_mod(self.wis),
            int : Self::compute_mod(self.int),
            cha : Self::compute_mod(self.cha), }
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

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { Character{ name : name, attributes : Attributes::default() } }
    /// Returns the name of the character as a string so it can be manipulated
    /// and stored without holding a reference to the character themself.
    pub fn name(&self) -> String { self.name.clone() }
    /// Sets the whole array of attributes at once
    pub fn set_attributes(&mut self, attrs : Attributes) { self.attributes = attrs }
    /// Gets the current attribute modifiers for this character
    pub fn modifiers(&self) -> Attributes { self.attributes.get_modifiers() }
}

#[cfg(test)]
mod test_character {
    use super::*;
    #[test]
    fn test_create_with_name() {
        let ch = Character::new(String::from("Dude"));
        assert_eq!(ch.name(), "Dude");
    }
    #[test]
    fn test_set_attributes() {
        let mut ch = Character::new(String::from("Gal"));
        ch.set_attributes( Attributes::default() );
        assert_eq!(ch.attributes, Attributes::default());
    }
    #[test]
    fn test_get_modifiers() {
        let ch = Character::new(String::from("Hunk"));
        assert_eq!(ch.modifiers(), Attributes{ str:0, dex:0, con:0, wis:0, int:0, cha:0 } );
    }
    #[test]
    fn test_modifiers() {
        let mut ch = Character::new(String::from("Babe"));
        ch.set_attributes( Attributes { str:9, dex:10, con:11, wis:12, int:13, cha:7 } );
        assert_eq!(ch.modifiers(), Attributes { str:-1, dex:0, con:0, wis:1, int:1, cha:-2 } );
    }
}