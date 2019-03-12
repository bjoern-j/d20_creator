pub mod attributes;
use super::HashMap;

pub struct Character {
    pub(super) name : Option<String>,
    pub(super) attributes : HashMap<attributes::Attribute, attributes::Value>,
}

impl Character {
    pub(super) fn new() -> Self {
        Character {
            name : None,
            attributes : attributes::Attribute::array_of_ten(),
        }
    }
    pub fn name(&self) -> &str { 
        match &self.name {
            None => "",
            Some(name) => &name,
        }
    }
    pub fn attribute(&self, attribute : attributes::Attribute) -> attributes::Value {
        match self.attributes.get(&attribute) {
            None => 0,
            Some(val) => *val,
        }
    }
}