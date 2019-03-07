#[cfg(test)]
mod test_character {
    use crate::character::*;
    use std::iter::FromIterator;
    #[test]
    fn test_create_with_name() {
        let ch = Character::new(String::from("Dude"));
        assert_eq!(ch.name(), "Dude");
    }
    #[test]
    fn test_set_attributes() {
        let mut ch = Character::new(String::from("Gal"));
        ch.set_attributes( Attributes::default() );
        assert_eq!(ch.attributes(), Attributes::default());
    }
    #[test]
    fn test_get_modifiers() {
        let ch = Character::new(String::from("Hunk"));
        assert_eq!(ch.modifiers(), Attributes::new(0,0,0,0,0,0) );
    }
    #[test]
    fn test_modifiers() {
        let mut ch = Character::new(String::from("Babe"));
        ch.set_attributes( Attributes::new(9,10,11,12,13,7) );
        assert_eq!(ch.modifiers(), Attributes::new(-1,0,0,1,1,-2) );
    }
    #[test]
    fn test_set_class() {
        let ch = get_warrior();
        assert_eq!(ch.save_mod(AttributeName::Str), 2);
    }
    #[test]
    fn test_hit_die() {
        let ch = get_warrior();
        assert_eq!(ch.hit_die(), DiceSize::D10);
    }
    #[test]
    fn test_set_race() {
        let ch = get_orc();
        assert_eq!(ch.attributes(), Attributes::new(12,10,11,10,10,10));
    }
    #[test]
    fn test_speed() {
        let ch = get_orc();
        assert_eq!(ch.speed(), 35);
    }
    #[test]
    fn test_skills() {
        let eat = Skill::new(String::from("Eat"), AttributeName::Con);
        let mut ch = Character::new(String::from("Munchie"));
        ch.set_attribute(AttributeName::Con, 16);
        assert_eq!(ch.skill_mod(&eat), 3);
        ch.add_skill_proficiency(&eat);
        assert_eq!(ch.skill_mod(&eat), 5);
    }
    #[test]
    fn test_weapon_proficiency() {
        let sword = Weapon::new(String::from("Sword"), vec![DiceSize::D8], DamageType::Piercing, WeaponCategory::Martial, WeaponType::Melee);
        let mut ch = Character::new(String::from("Lady of Swords"));
        ch.set_attribute(AttributeName::Str, 18);
        assert_eq!(ch.weapon_attack_mod(&sword), 4);
        ch.add_weapon_proficiency(&sword);
        assert_eq!(ch.weapon_attack_mod(&sword), 6)
    }
    #[test]
    fn test_ranged_weapon_proficiency() {
        let bow = Weapon::new(String::from("Bow"), vec![DiceSize::D10], DamageType::Piercing, WeaponCategory::Martial, WeaponType::Ranged);
        let mut ch = Character::new(String::from("Legolas"));
        ch.set_attribute(AttributeName::Dex, 14);
        assert_eq!(ch.weapon_attack_mod(&bow), 2);
    }
    #[test]
    fn test_weapon_category_proficiency() {
        let sword = Weapon::new(String::from("Sword"), vec![DiceSize::D8], DamageType::Piercing, WeaponCategory::Martial, WeaponType::Melee);
        let mut ch = Character::new(String::from("Duke"));
        ch.add_weapon_category_proficiency(WeaponCategory::Martial);
        assert_eq!(ch.weapon_attack_mod(&sword), 2);
    }
    #[test]
    fn test_armor_proficiency() {
        let mut ch = Character::new(String::from("Tank"));
        ch.add_armor_proficiency(ArmorCategory::Medium);
        assert_eq!(ch.proficient_with(ArmorCategory::Medium), true);
    }
    #[test]
    fn test_alignment() {
        let mut ch = Character::new(String::from("Evulz"));
        ch.set_alignment(Alignment::ChaoticEvil);
        assert_eq!(ch.alignment(), Alignment::ChaoticEvil);
    }
    #[test]
    fn test_size() {
        let ch = get_orc();
        assert_eq!(ch.size(), Size::Small);
    }
    #[test]
    fn test_spells() {
        let ch = get_mage();
        assert_eq!(ch.spells_at_level(9, SpellLevel::Second), 3);
        let ch = get_warrior();
        assert_eq!(ch.spells_at_level(3, SpellLevel::Cantrip), 0);
    }
    #[test]
    fn test_subclasses() {
        let mut ch = get_mage();
        assert!(ch.set_subclass("School of Abjuration"));
        assert!(!ch.set_subclass("School of Hard Knocks"));
    }
    #[test]
    fn test_subraces() {
        let mut ch = get_gith();
        assert!(ch.set_subrace("Slave"));
        assert_eq!(ch.attributes().get(AttributeName::Int), 11);
    }
    #[test]
    fn test_non_mechanical_trait() {
        let mut ch = Character::new(String::from("Blank"));
        let feat = Feat{
            name : String::from("Lucky"),
            text : String::from("This character is lucky."),
            effect : None,
        };
        feat.apply(&mut ch);
        assert!(ch.has_feat("Lucky"));
    }
    #[test]
    fn test_mechanical_trait() {
        let mut ch = Character::new(String::from("Grog"));
        let feat = Feat{
            name : String::from("Strong"),
            text : String::from("This character is strong"),
            effect : Some(
                Box::new(|ch : &mut Character| ( ch.set_attribute(AttributeName::Str, ch.attributes().get(AttributeName::Str) + 2) ) )
            ),
        };
        feat.apply(&mut ch);
        assert_eq!(ch.attributes().get(AttributeName::Str), 12);
    }

    fn get_gith() -> Character {
        let gith = Race { 
            name : String::from("Gith"),
            speed : 30,
            size : Size::Medium,
            attribute_bonuses : HashMap::from_iter([(AttributeName::Dex, 2)].iter().cloned()),
            subraces : HashMap::from_iter([
                (String::from("Slave"), Subrace {
                    name : String::from("Slave"),
                    attribute_bonuses : HashMap::from_iter([(AttributeName::Int, 1)].iter().cloned())
                })
            ].iter().cloned()),
        };
        let mut ch = Character::new(String::from("Dak'kon"));
        ch.set_race(gith);
        ch
    }

    fn get_mage_class() -> Class {
        Class{
            name : String::from("Mage"),
            save_proficiencies : HashSet::from_iter([AttributeName::Wis, AttributeName::Int].iter().cloned()),
            hit_die : DiceSize::D6,
            subclasses : HashMap::from_iter([
                (String::from("School of Abjuration"), Subclass{ name : String::from("School of Abjuration") } ), 
                (String::from("School of Conjuration"), Subclass{ name : String::from("School of Conjuration") } )
            ].iter().cloned()),
            spells : HashMap::from_iter([
                (9, HashMap::from_iter([
                    (SpellLevel::Cantrip, 4),
                    (SpellLevel::First, 4),
                    (SpellLevel::Second, 3),
                    (SpellLevel::Third, 3),
                    (SpellLevel::Fourth, 4),
                    (SpellLevel::Fifth, 5)
                ].iter().cloned()))
            ].iter().cloned()),
        }
    }

    fn get_mage() -> Character {
        let mage = get_mage_class();
        let mut ch = Character::new(String::from("Raistlin"));
        ch.set_class(mage);
        ch
    }

    fn get_warrior() -> Character {
        let warrior = Class{ 
            name : String::from("Warrior"), 
            save_proficiencies : HashSet::from_iter([AttributeName::Str, AttributeName::Con].iter().cloned()),  
            hit_die : DiceSize::D10,
            subclasses : HashMap::new(),
            spells : HashMap::from_iter([].iter().cloned())
        };
        let mut ch = Character::new(String::from("Vala"));
        ch.set_class(warrior);
        ch
    }
    fn get_orc() -> Character {
        let orc = Race{
            name : String::from("Orc"),
            speed : 35,
            size : Size::Small,
            attribute_bonuses : HashMap::from_iter([(AttributeName::Str, 2), (AttributeName::Con, 1)].iter().cloned()),
            subraces : HashMap::new(),
        };
        let mut ch = Character::new(String::from("Hruumsh"));
        ch.set_race(orc);
        ch
    }
}