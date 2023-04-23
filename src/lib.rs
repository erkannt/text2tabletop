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
        name: parse_name(&input).to_string(),
        points: parse_points(&input).to_string(),
        system: parse_system(&input).to_string(),
        units: parse_units(&input),
    }
}

fn parse_name(input: &str) -> &str {
    let re = Regex::new(r"^\+\+ (.*) \[").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract name]")
}

fn parse_points(input: &str) -> &str {
    let re = Regex::new(r"([\d]+)pts\] \+\+").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract points]")
}

fn parse_system(input: &str) -> &str {
    let re = Regex::new(r"\[([[:alpha:]]+) ").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract system]")
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
                state.completed.push(Unit { name: partial.0 })
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
            C!["paste-area"],
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
