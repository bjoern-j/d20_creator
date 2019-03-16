use super::*;

pub struct Race {
    pub(super) name : String,
    pub(super) attributes : HashMap<Attribute, AttributeValue>,
    pub(super) size : Size,
    pub(super) speed : Speed
}

impl Race {
    pub fn new(name : String, attributes : HashMap<Attribute, AttributeValue>, size : Size, speed : Speed) -> Self {
        Race { name : name, attributes : attributes, size : size, speed : speed }
    }
    pub fn name(&self) -> &str { &self.name }
}