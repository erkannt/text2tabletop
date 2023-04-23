#![allow(clippy::wildcard_imports)]

use askama::Template;
use regex::Regex;
use seed::{prelude::*, *};
use serde::Deserialize;

const STORAGE_KEY: &str = "text2tabletop";

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        stored_input: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        army_list: LocalStorage::get(STORAGE_KEY)
            .ok()
            .map(|input: String| parse_army_list(&input)),
    }
}

struct Model {
    stored_input: String,
    army_list: Option<ArmyList>,
}

#[derive(Template, Deserialize)]
#[template(path = "army-list.html")]
struct ArmyList {
    name: String,
    points: String,
    system: String,
    units: Vec<Unit>,
}

#[derive(Deserialize)]
struct Unit {
    name: String,
    count: String,
    models: String,
    points: String,
    special_rules: String,
    quality: String,
    defense: String,
    weapons: Vec<String>,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
enum Msg {
    ArmyUpdated(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArmyUpdated(input) => {
            model.army_list = Some(parse_army_list(&input));
            LocalStorage::insert(STORAGE_KEY, &input).expect("save to LocalStorage failed");
            model.stored_input = input;
        }
    }
}

fn parse_army_list(input: &String) -> ArmyList {
    ArmyList {
        name: extract_single("name", Regex::new(r"^\+\+ (.*) \[").unwrap(), &input),
        points: extract_single("point", Regex::new(r"([\d]+)pts\] \+\+").unwrap(), &input),
        system: extract_single("system", Regex::new(r"\[([[:alpha:]]+) ").unwrap(), &input),
        units: parse_units(&input),
    }
}

fn extract_single_or(name: &str, re: Regex, input: &str, default: &str) -> String {
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .map(|s| s.to_string())
        .unwrap_or(default.to_string())
}

fn extract_single(name: &str, re: Regex, input: &str) -> String {
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .map(|s| s.to_string())
        .unwrap_or(format!("[error: can't extract {}]", name))
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
                    count: extract_single_or(
                        "count",
                        Regex::new(r"^([\d+])x ").unwrap(),
                        &partial.0,
                        "1",
                    ),
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
                    weapons: line
                        .split("), ")
                        .into_iter()
                        .map(|extract| format!("{})", extract))
                        .collect(),
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

fn view(model: &Model) -> Node<Msg> {
    let rendered_list = match &model.army_list {
        Some(a) => a.render().unwrap_or_else(|err| format!("{}", err)),
        None => "".to_string(),
    };
    div![
        textarea![
            C!["paste-area", "inputs"],
            attrs! {At::Rows => 10, At::Value => model.stored_input},
            input_ev(Ev::Change, Msg::ArmyUpdated),
            input_ev(Ev::KeyUp, Msg::ArmyUpdated)
        ],
        raw!(&rendered_list)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
