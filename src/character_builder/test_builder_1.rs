use super::*;
use std::iter::FromIterator;
#[test]
fn test_name() {
    let mut builder = Builder::new();
    builder.set_name("John".to_owned());
    assert_eq!(builder.character().name(), "John");
}
#[test]
fn test_attributes() {
    let mut builder = Builder::new();
    builder.set_attribute(Attribute::Str, 15);
    assert_eq!(builder.character().attribute(Attribute::Str), 15);
    assert_eq!(builder.character().attribute(Attribute::Dex), 10);
}
#[test]
fn test_race() {
    let mut builder = Builder::new();
    let elf = get_elf_race();
    builder.add_race(elf);
    builder.set_race("Elf");
    assert_eq!(builder.character().attribute(Attribute::Dex), 12);
    assert_eq!(builder.character().attribute(Attribute::Int), 11);
}
#[test]
fn test_unset_race() {
    let mut builder = Builder::new();
    builder.add_race(get_elf_race());
    builder.add_race(get_dwarf_race());
    builder.set_race("Elf");
    builder.set_race("Dwarf");
    assert_eq!(builder.character().attribute(Attribute::Str), 12);
    assert_eq!(builder.character().attribute(Attribute::Dex), 10);
    assert_eq!(builder.character().attribute(Attribute::Int), 9);
}

fn get_elf_race() -> Race {
    Race::new(
        "Elf".to_owned(),
        HashMap::from_iter(
            [(Attribute::Dex, 2), (Attribute::Int, 1)].iter().cloned()
        )
    )
}

fn get_dwarf_race() -> Race {
    Race::new(
        "Dwarf".to_owned(),
        HashMap::from_iter(
            [(Attribute::Str, 2), (Attribute::Int, -1)].iter().cloned()
        )
    )
}