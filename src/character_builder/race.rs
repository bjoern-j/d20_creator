use super::*;

pub struct Race {
    pub(super) name : String,
    pub(super) attributes : HashMap<Attribute, AttributeValue>,
    pub(super) size : Size,
    pub(super) speed : Speed,
    pub(super) feats : Vec<String>,
}

impl Race {
    pub fn new(name : String, attributes : HashMap<Attribute, AttributeValue>, size : Size, speed : Speed, feats : Vec<String>) -> Self {
        Race { name : name, attributes : attributes, size : size, speed : speed, feats : feats }
    }
    pub fn name(&self) -> &str { &self.name }
}