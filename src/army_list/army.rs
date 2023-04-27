use regex::Regex;

use super::regex_helpers::{extract_single, extract_single_or};
use super::weapons::{parse_weapons, Weapon};

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
