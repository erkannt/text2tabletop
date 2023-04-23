#![allow(clippy::wildcard_imports)]

use regex::Regex;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { army_list: None }
}

struct Model {
    army_list: Option<ArmyList>,
}

struct ArmyList {
    name: String,
    points: u16,
    system: String,
    rest: String,
}

#[derive(Clone)]
enum Msg {
    ArmyUpdated(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArmyUpdated(input) => {
            model.army_list = Some(ArmyList {
                name: parse_name(&input).to_string(),
                points: 0,
                system: "".to_string(),
                rest: input,
            })
        }
    }
}

fn parse_name(input: &String) -> &str {
    let re = Regex::new(r"\[?([A-Za-z]+)").unwrap();
    re.find(input)
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract name]")
}

fn view(model: &Model) -> Node<Msg> {
    let rendered_list = match &model.army_list {
        Some(a) => format!("<h1>{}</h1><p>{}</p>", a.name, a.rest),
        None => "".to_string(),
    };
    div![
        textarea![
            C!["paste-area"],
            attrs! {At::Rows => 10},
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
