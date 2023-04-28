use std::convert::identity;

use regex::Regex;

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub value: String,
}

pub fn parse_definitions(input: &str) -> Option<Vec<Definition>> {
    if input.trim().is_empty() {
        return None;
    }
    let spells = input
        .lines()
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(parse_definition)
        .filter_map(identity)
        .collect();
    Some(spells)
}

fn parse_definition(line: &str) -> Option<Definition> {
    let captures = Regex::new(r"^(.+): (.*)$").unwrap().captures(line);

    let name = captures
        .as_ref()
        .and_then(|cap| cap.get(1))
        .and_then(|mat| line.get(mat.range()))
        .map(str::to_string);

    let value = captures
        .and_then(|cap| cap.get(2))
        .and_then(|mat| line.get(mat.range()))
        .map(str::to_string);

    if let (Some(name), Some(value)) = (name, value) {
        return Some(Definition { name, value });
    }

    None
}
