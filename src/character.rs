pub struct Character {
    name : String,
}

impl Character {
    /// Creates a new character with a name and nothing else
    pub fn new(name : String) -> Self { Character{ name : name } }
    /// Returns the name of the character as a string so it can be manipulated
    /// and stored without holding a reference to the character themself.
    pub fn name(&self) -> String { self.name.clone() }
}

#[cfg(test)]
mod test_character {
    use super::*;
    #[test]
    fn test_create_with_name() {
        let ch = Character::new(String::from("Dude"));
        assert_eq!(ch.name(), "Dude");
    }
}