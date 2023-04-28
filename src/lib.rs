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
        notes: render_notes(&inputs.notes),
        inputs,
    }
}

struct Model {
    inputs: Inputs,
    army_list_view_model: Option<ArmyListViewModel>,
    notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Inputs {
    army: String,
    spells: String,
    rules: String,
    notes: String,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            army: Default::default(),
            spells: Default::default(),
            rules: Default::default(),
            notes: Default::default(),
        }
    }
}

#[derive(Template, Debug)]
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
    NotesUpdated(String),
    SeeAnExample,
    ClearAll,
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
        Msg::NotesUpdated(notes_input) => {
            let old_inputs = model.inputs.clone();
            model.inputs = Inputs {
                notes: notes_input,
                ..old_inputs
            };
            model.notes = render_notes(&model.inputs.notes);
        }
        Msg::SeeAnExample => {
            model.inputs = Inputs {
                army: include_str!("../static/example-army.txt").to_string(),
                spells: include_str!("../static/example-spells.txt").to_string(),
                rules: include_str!("../static/example-rules.txt").to_string(),
                notes: "You can use markdown to write notes.\n\n### A heading\n\n- a bullet point\n- another with _italics_".to_string()
            };
            model.army_list_view_model = parse_army_list_view_model(&model.inputs);
        }
        Msg::ClearAll => {
            model.inputs = Inputs::default();
            window().location().reload().unwrap_or_default();
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

fn render_notes(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    };
    let parser = pulldown_cmark::Parser::new(input);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    return Some(html_output);
}

fn view(model: &Model) -> Node<Msg> {
    let rendered_list = match &model.army_list_view_model {
        Some(view_model) => view_model.render().unwrap_or_else(|err| format!("{}", err)),
        None => "".to_string(),
    };

    let rendered_notes_section = match &model.notes {
        Some(notes) => format!("<section class=\"notes\"><h2>Notes</h2>{}</section>", notes),
        None => "".to_string(),
    };

    div![
        nav![
            C!["inputs", "input-helpers"],
            a![
                attrs![At::Href => "#"],
                input_ev(Ev::Click, |_| Msg::SeeAnExample),
                "See an example"
            ],
            a![
                attrs![At::Href => "#"],
                input_ev(Ev::Click, |_| Msg::ClearAll),
                "Clear all"
            ],
        ],
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
            label![attrs![At::For => "notes"], "Notes"],
            textarea![
                C!["paste-area", "input"],
                attrs! {At::Id => "notes", At::Rows => 10},
                input_ev(Ev::Change, Msg::NotesUpdated),
                input_ev(Ev::KeyUp, Msg::NotesUpdated),
                model.inputs.notes.clone()
            ],
        ],
        raw!(&rendered_list),
        raw!(&rendered_notes_section)
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
