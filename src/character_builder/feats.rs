pub trait Feat {
    fn name(&self) -> &str;
}

pub struct NonMechanicalFeat {
    pub name : String,
    pub long_text : String,
}

impl Feat for NonMechanicalFeat {
    fn name(&self) -> &str { &self.name }
}