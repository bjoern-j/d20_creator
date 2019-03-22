use super::Character;

pub trait Feat {
    fn name(&self) -> &str;
    fn apply_to(&self, ch : &mut Character);
}

pub struct NonMechanicalFeat {
    pub name : String,
    pub long_text : String,
}

impl Feat for NonMechanicalFeat {
    fn name(&self) -> &str { &self.name }
    fn apply_to(&self, _ : &mut Character) { () }
}

pub struct MechanicalFeat {
    pub name : String,
    pub long_text : String,
    pub effect : Box<Fn(&mut Character)>,
}

impl Feat for MechanicalFeat {
    fn name(&self) -> &str { &self.name }
    fn apply_to(&self, ch : &mut Character) {
        (self.effect)(ch)
    }
}