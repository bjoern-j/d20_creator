
#[derive(PartialEq, Eq, Hash)]
#[derive(Copy, Clone)]
pub enum WeaponCategory{
    Simple,
    Martial,
}

pub struct Weapon {
    name : String,
}

impl Weapon {
    pub fn new(name : String) -> Self {
        Weapon { name : name }
    }
    pub fn name(&self) -> &str { &self.name }
}