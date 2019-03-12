use super::*;
use character::attributes::Attribute;
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