use super::character::DiceSize;

pub type WeaponName = String;

pub struct Weapon {
    name : WeaponName,
    dice : Vec<DiceSize>,
    damage_type : DamageType,
    category : WeaponCategory,
    weapon_type : WeaponType,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
pub enum DamageType{
    Slashing,
    Piercing,
    Bludgeoning,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
pub enum WeaponCategory{
    Simple,
    Martial,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq,Hash)]
#[derive(Debug)]
pub enum WeaponType{
    Melee,
    Ranged,
}

impl Weapon {
    /// Returns a new weapon with the specified attributes
    pub fn new(name : WeaponName, dice : Vec<DiceSize>, damage_type : DamageType, category : WeaponCategory, weapon_type : WeaponType ) -> Self {
        Weapon{ name : name, dice : dice, damage_type : damage_type, category : category, weapon_type : weapon_type }
    }
    /// Returns the weapon's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Returns this weapon's category
    pub fn category(&self) -> WeaponCategory { self.category }
    /// Returns this weapon's type
    pub fn weapon_type(&self) -> WeaponType { self.weapon_type }
}