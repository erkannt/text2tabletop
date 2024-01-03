use regex::Regex;

use crate::army_list::regex_helpers::extract_optional_single;

use super::regex_helpers::{extract_single, extract_single_or};
use super::weapons::{parse_weapons, Weapon};

#[derive(Debug)]
pub struct Army {
    pub name: String,
    pub points: String,
    pub system: String,
    pub units: Vec<Unit>,
}

#[derive(Debug, PartialEq)]
pub struct Unit {
    pub name: String,
    pub count: String,
    pub models: String,
    pub points: String,
    pub xp: Option<String>,
    pub special_rules: String,
    pub quality: String,
    pub defense: String,
    pub weapons: Vec<Weapon>,
    pub joined_to: Option<String>,
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
        if line.starts_with("#") {
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
                    count: extract_single_or("1", Regex::new(r"^(\d+)x ").unwrap(), &partial.0),
                    models: extract_single("models", Regex::new(r"\[(\d+)\]").unwrap(), &partial.0),
                    points: extract_single("points", Regex::new(r"(\d+)pts").unwrap(), &partial.0),
                    xp: extract_optional_single(Regex::new(r"(\d+)XP").unwrap(), &partial.0),
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
                    joined_to: None,
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

#[cfg(test)]
mod tests {
    use crate::army_list::{
        army::{parse_units, Unit},
        weapons::Weapon,
    };

    #[test]
    fn unjoined_unit() {
        let parsed = parse_units(
            "2x Drained Soldiers [10] Q5+ D5+ | 85pts | Undead
                    10x Spear (A1, Counter)",
        );
        let expected = Unit {
            name: "Drained Soldiers".to_string(),
            count: "2".to_string(),
            models: "10".to_string(),
            points: "85".to_string(),
            xp: None,
            special_rules: "Undead".to_string(),
            quality: "5".to_string(),
            defense: "5".to_string(),
            weapons: vec![Weapon::Melee("10x Spear (A1, Counter)".to_string())],
            joined_to: None,
        };
        assert_eq!(parsed[0], expected)
    }

    #[test]
    fn joined_unit() {
        let parsed = parse_units(
            "Champion [1] Q4+ D4+ | 95pts | Hero, Tough(3), Undead, 1x Master Necromancer(Caster(3))
                    Hand Weapon (A3)
                    | Joined to:
                    Drained Archers [10] Q5+ D5+ | 135pts | Undead, Banner
                    10x Bow (24\", A1), 10x Hand Weapon (A1)
                    ",
        );
        let expected = Unit {
            name: "Champion".to_string(),
            count: "1".to_string(),
            models: "1".to_string(),
            points: "95".to_string(),
            xp: None,
            special_rules: "Hero, Tough(3), Undead, 1x Master Necromancer(Caster(3))".to_string(),
            quality: "4".to_string(),
            defense: "4".to_string(),
            weapons: vec![Weapon::Melee("Hand Weapon (A3)".to_string())],
            joined_to: Some("Drained Archers".to_string()),
        };
        assert_eq!(parsed[0], expected)
    }
}
