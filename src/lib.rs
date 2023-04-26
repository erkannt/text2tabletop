#![allow(clippy::wildcard_imports)]

use askama::Template;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod army_list;

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
    rules: String,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            army: Default::default(),
            spells: Default::default(),
            rules: Default::default(),
        }
    }
}

#[derive(Template)]
#[template(path = "army-list.html")]
struct ArmyListViewModel {
    army: army_list::Army,
    spells: Option<Vec<army_list::Definition>>,
    rules: Option<Vec<army_list::Definition>>,
}

#[derive(Clone)]
enum Msg {
    ArmyUpdated(String),
    SpellsUpdated(String),
    RulesUpdated(String),
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
            model.army_list_view_model = parse_army_list_view_model(&model.inputs);
        }
        Msg::RulesUpdated(rules_input) => {
            let old_inputs = model.inputs.clone();
            model.inputs = Inputs {
                rules: rules_input,
                ..old_inputs
            };
            model.army_list_view_model = parse_army_list_view_model(&model.inputs);
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.inputs).expect("save to LocalStorage failed");
}

fn parse_army_list_view_model(inputs: &Inputs) -> Option<ArmyListViewModel> {
    if inputs.army.trim().is_empty() {
        return None;
    }

    return Some(ArmyListViewModel {
        army: army_list::parse_army(&inputs.army),
        spells: army_list::parse_definitions(&inputs.spells),
        rules: army_list::parse_definitions(&inputs.rules),
    });
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
                attrs! {At::Id => "list", At::Rows => 20},
                input_ev(Ev::Change, Msg::ArmyUpdated),
                input_ev(Ev::KeyUp, Msg::ArmyUpdated),
                model.inputs.army.clone()
            ],
            label![attrs![At::For => "spells"], "Spells"],
            textarea![
                C!["paste-area", "input"],
                attrs! {At::Id => "spells", At::Rows => 13},
                input_ev(Ev::Change, Msg::SpellsUpdated),
                input_ev(Ev::KeyUp, Msg::SpellsUpdated),
                model.inputs.spells.clone()
            ],
            label![attrs![At::For => "rules"], "Rules"],
            textarea![
                C!["paste-area", "input"],
                attrs! {At::Id => "rules", At::Rows => 10},
                input_ev(Ev::Change, Msg::RulesUpdated),
                input_ev(Ev::KeyUp, Msg::RulesUpdated),
                model.inputs.rules.clone()
            ],
        ],
        raw!(&rendered_list)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
