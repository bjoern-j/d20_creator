mod character;
mod attributes;
mod weapons;
mod data_store;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::iter::FromIterator;
use attributes::AttributeName;

fn main() {
    let human = character::Race{ 
        name : "Human".to_owned(), 
        speed : 30,
        attribute_bonuses : HashMap::from_iter([(AttributeName::Str, 1), (AttributeName::Dex, 1), (AttributeName::Con, 1),
                                                (AttributeName::Wis, 1), (AttributeName::Int, 1), (AttributeName::Cha, 1)].iter().cloned()) };
    let race_json = serde_json::to_string(&human);
    println!("{}", race_json.unwrap());
    let json_race : character::Race = serde_json::from_str(r#"{"name":"Human","speed":30,"attribute_bonuses":{"Con":1,"Cha":1,"Str":1,"Dex":1,"Wis":1,"Int":1}}"#).unwrap();
    println!("{}", serde_json::to_string(&json_race).unwrap())
}
