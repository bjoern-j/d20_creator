use super::*;

pub struct Race {
    name : String,
    pub(super) attributes : HashMap<Attribute, AttributeValue>,
}

impl Race {
    pub fn new(name : String, attributes : HashMap<Attribute, AttributeValue>) -> Self {
        Race { name : name, attributes : attributes }
    }
    pub fn name(&self) -> &str { &self.name }
}