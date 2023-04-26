use regex::Regex;

use crate::army_list::regex_helpers::extract_single_or;

use super::regex_helpers::extract_single;

pub struct Army {
    pub name: String,
    pub points: String,
    pub system: String,
    pub units: Vec<Unit>,
}

pub struct Unit {
    pub name: String,
    pub count: String,
    pub models: String,
    pub points: String,
    pub special_rules: String,
    pub quality: String,
    pub defense: String,
    pub weapons: Vec<Weapon>,
}

#[derive(Debug, PartialEq)]
pub enum Weapon {
    Melee(String),
    Ranged(String),
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::Melee(value) => write!(f, "⚔ {}", value),
            Weapon::Ranged(value) => write!(f, "➶ {}", value),
        }
    }
}

pub fn parse_army(input: &String) -> Army {
    Army {
        name: extract_single("name", Regex::new(r"^\+\+ (.*) \[").unwrap(), &input),
        points: extract_single("point", Regex::new(r"([\d]+)pts\] \+\+").unwrap(), &input),
        system: extract_single("system", Regex::new(r"\[([[:alpha:]]+) ").unwrap(), &input),
        units: parse_units(&input),
    }
}

fn parse_units(input: &str) -> Vec<Unit> {
    struct State {
        partial: Option<PartialUnit>,
        completed: Vec<Unit>,
    }

    struct PartialUnit(String);

    fn handle_line(mut state: State, line: &str) -> State {
        if line.starts_with("++") {
            return state;
        }
        if line.is_empty() {
            return state;
        }
        match state.partial {
            None => state.partial = Some(PartialUnit(line.to_string())),
            Some(partial) => {
                state.partial = None;
                state.completed.push(Unit {
                    name: extract_single(
                        "name",
                        Regex::new(r"^(?:[\d+]x )?([^\[]+) \[").unwrap(),
                        &partial.0,
                    ),
                    count: extract_single_or("1", Regex::new(r"^([\d+])x ").unwrap(), &partial.0),
                    models: extract_single("models", Regex::new(r"\[(\d+)\]").unwrap(), &partial.0),
                    points: extract_single("points", Regex::new(r"(\d+)pts").unwrap(), &partial.0),
                    quality: extract_single("quality", Regex::new(r"Q(\d)\+").unwrap(), &partial.0),
                    defense: extract_single(
                        "defense",
                        Regex::new(r"D(\d+)\+").unwrap(),
                        &partial.0,
                    ),
                    special_rules: extract_single(
                        "special_rules",
                        Regex::new(r"^.*\|.*\| (.*)$").unwrap(),
                        &partial.0,
                    ),
                    weapons: parse_weapons(line),
                })
            }
        }
        return state;
    }

    let result = input.lines().fold(
        State {
            partial: None,
            completed: vec![],
        },
        handle_line,
    );

    return result.completed;
}

fn parse_weapons(line: &str) -> Vec<Weapon> {
    let re =
        Regex::new(r"(?:\d+x )??[A-Za-z -]+ \((?:\d+., )??A\d(?:, AP\(\d\))??(?:, [A-ZA-z ]+)??\)")
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
    use crate::army_list::army::{parse_weapons, Weapon};

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
}
