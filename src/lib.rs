#![allow(clippy::wildcard_imports)]

use askama::Template;
use regex::Regex;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "text2tabletop";

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let inputs: Inputs = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    Model {
        army_list_view_model: parse_army_list_view_model(&inputs),
        inputs,
    }
}

struct Model {
    inputs: Inputs,
    army_list_view_model: Option<ArmyListViewModel>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Inputs {
    army: String,
    spells: String,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            army: Default::default(),
            spells: Default::default(),
        }
    }
}

#[derive(Template)]
#[template(path = "army-list.html")]
struct ArmyListViewModel {
    army: Army,
    spells: Option<Vec<String>>,
}

struct Army {
    name: String,
    points: String,
    system: String,
    units: Vec<Unit>,
}

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

#[derive(Clone)]
enum Msg {
    ArmyUpdated(String),
    SpellsUpdated(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArmyUpdated(army_input) => {
            let old_inputs = model.inputs.clone();
            model.inputs = Inputs {
                army: army_input,
                ..old_inputs
            };
            model.army_list_view_model = parse_army_list_view_model(&model.inputs);
        }
        Msg::SpellsUpdated(spells_input) => {
            let old_inputs = model.inputs.clone();
            model.inputs = Inputs {
                spells: spells_input,
                ..old_inputs
            };
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.inputs).expect("save to LocalStorage failed");
}

fn parse_army_list_view_model(inputs: &Inputs) -> Option<ArmyListViewModel> {
    if inputs.army.trim().is_empty() {
        return None;
    }
    if inputs.spells.trim().is_empty() {
        return Some(ArmyListViewModel {
            army: parse_army(&inputs.army),
            spells: None,
        });
    }
    return Some(ArmyListViewModel {
        army: parse_army(&inputs.army),
        spells: Some(parse_spells(&inputs.spells)),
    });
}

fn parse_army(input: &String) -> Army {
    Army {
        name: extract_single("name", Regex::new(r"^\+\+ (.*) \[").unwrap(), &input),
        points: extract_single("point", Regex::new(r"([\d]+)pts\] \+\+").unwrap(), &input),
        system: extract_single("system", Regex::new(r"\[([[:alpha:]]+) ").unwrap(), &input),
        units: parse_units(&input),
    }
}

fn extract_single_or(default: &str, re: Regex, input: &str) -> String {
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

fn parse_spells(input: &str) -> Vec<String> {
    input
        .lines()
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect()
}

fn view(model: &Model) -> Node<Msg> {
    let rendered_list = match &model.army_list_view_model {
        Some(view_model) => view_model.render().unwrap_or_else(|err| format!("{}", err)),
        None => "".to_string(),
    };
    div![
        div![
            C!["inputs"],
            label![attrs![At::For => "list"], "Army"],
            textarea![
                C!["paste-area", "input"],
                attrs! {At::Id => "list", At::Rows => 20, At::Value => model.inputs.army},
                input_ev(Ev::Change, Msg::ArmyUpdated),
                input_ev(Ev::KeyUp, Msg::ArmyUpdated)
            ],
            label![attrs![At::For => "spells"], "Spells"],
            textarea![
                C!["paste-area", "input"],
                attrs! {At::Id => "spells", At::Rows => 10, At::Value => model.inputs.spells},
                input_ev(Ev::Change, Msg::SpellsUpdated),
                input_ev(Ev::KeyUp, Msg::SpellsUpdated)
            ],
        ],
        raw!(&rendered_list)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
