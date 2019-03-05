use std::collections::HashMap;
use crate::character::Race;

pub struct DataStore {
    races : Races,
}

pub type Races = HashMap<String, Race>;

impl DataStore {
    pub fn new() -> Self {
        DataStore{ 
            races : HashMap::new(),
        }
    }
    pub fn races(&self) -> &Races { &self.races }
    pub fn add_race_from_json(&mut self, json : String) {
        let race : Race = serde_json::from_str(json.as_str()).expect("Invalid JSON for race.");
        self.races.insert(race.name.clone(), race);
    }
}

#[cfg(test)]
mod test_parse_from_json {
    use super::*;
    use crate::attributes::AttributeName;
    #[test]
    fn test_race() {
        let mut store = DataStore::new();
        store.add_race_from_json(String::from(
            r#"{"name":"Human","speed":30,"attribute_bonuses":{"Con":1,"Cha":1,"Str":1,"Dex":1,"Wis":1,"Int":1}}"#));
        let races = store.races();
        let human = races.get("Human").unwrap();
        assert_eq!(human.name, "Human");
        assert_eq!(human.speed, 30);
        assert_eq!(*human.attribute_bonuses.get(&AttributeName::Str).unwrap(), 1);
    }
}