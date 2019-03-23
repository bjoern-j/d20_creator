use super::*;

pub struct Race {
    pub(super) name : String,
    pub(super) attributes : AttributeArray,
    pub(super) size : Size,
    pub(super) speed : Speed,
    pub(super) feats : Vec<String>,
    pub(super) subraces : HashMap<String, Subrace>,
}

pub(super) struct Subrace {
    pub(super) name : String,
    pub(super) attributes : AttributeArray,
    pub(super) feats : Vec<String>,
}

type AttributeArray = HashMap<Attribute, AttributeValue>;

impl Race {
    pub fn new(name : String, attributes : AttributeArray, size : Size, speed : Speed, feats : Vec<String>) -> Self {
        Race { name : name, attributes : attributes, size : size, speed : speed, feats : feats, subraces : HashMap::new() }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn add_subrace(&mut self, name : String, attributes : AttributeArray, feats : Vec<String>) {
        self.subraces.insert(
            name.clone(), 
            Subrace {
                name : name,
                attributes : attributes,
                feats : feats,
            }
        );
    }
}