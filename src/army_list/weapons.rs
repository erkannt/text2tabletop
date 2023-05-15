use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Weapon {
    Melee(String),
    Ranged(String),
}

pub fn parse_weapons(line: &str) -> Vec<Weapon> {
    let re =
        Regex::new(r"(?:\d+x )??[A-Za-z -]+ \((?:\d+., )??A\d(?:, AP\(\d\))??(?:, [A-ZA-z- ]+(?:\([0-9]+\))??)*\)")
            .unwrap();

    re.captures_iter(line)
        .map(|cap| cap[0].trim().to_string())
        .map(|weapon| match weapon.contains("\"") {
            true => Weapon::Ranged(weapon),
            false => Weapon::Melee(weapon),
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::{parse_weapons, Weapon};

    #[test]
    fn simple_weapon() {
        let parsed = parse_weapons("Hand Weapon (A3)");
        let expected = vec![Weapon::Melee("Hand Weapon (A3)".to_string())];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn weapon_with_armor_piercing() {
        let parsed = parse_weapons("Hand Weapon (A3, AP(1))");
        let expected = vec![Weapon::Melee("Hand Weapon (A3, AP(1))".to_string())];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn weapon_with_ap_and_rule() {
        let parsed = parse_weapons("Hand Weapon (A3, AP(1), Rending)");
        let expected = vec![Weapon::Melee(
            "Hand Weapon (A3, AP(1), Rending)".to_string(),
        )];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn multiple_weapons() {
        let parsed = parse_weapons("Gatling-Fists (18\", A3, AP(1)), Stomp (A2)");
        let expected = vec![
            Weapon::Ranged("Gatling-Fists (18\", A3, AP(1))".to_string()),
            Weapon::Melee("Stomp (A2)".to_string()),
        ];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn multiple_models() {
        let parsed = parse_weapons("2x Hand Weapon (A1)");
        let expected = vec![Weapon::Melee("2x Hand Weapon (A1)".to_string())];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn muliple_rules() {
        let parsed = parse_weapons("Power Staff (A3, Warp, Rending)");
        let expected = vec![Weapon::Melee("Power Staff (A3, Warp, Rending)".to_string())];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn rules_with_brackets() {
        let parsed = parse_weapons("CCW (A1), Fusion Rifle (12\", A1, AP(4), Deadly(3))");
        let expected = vec![
            Weapon::Melee("CCW (A1)".to_string()),
            Weapon::Ranged("Fusion Rifle (12\", A1, AP(4), Deadly(3))".to_string()),
        ];
        assert_eq!(parsed, expected)
    }

    #[test]
    fn rules_with_hyphens() {
        let parsed = parse_weapons("Bash (A2), Bludgeon (A1, Impact(1)), Missile Pod (18\", A1, AP(2), Lock-On), Power Claw (A1, AP(1), Rending)");
        let expected = vec![
            Weapon::Melee("Bash (A2)".to_string()),
            Weapon::Melee("Bludgeon (A1, Impact(1))".to_string()),
            Weapon::Ranged("Missile Pod (18\", A1, AP(2), Lock-On)".to_string()),
            Weapon::Melee("Power Claw (A1, AP(1), Rending)".to_string()),
        ];
        assert_eq!(parsed, expected)
    }
}
