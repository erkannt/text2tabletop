#![allow(clippy::wildcard_imports)]

use askama::Template;
use regex::Regex;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { army_list: None }
}

struct Model {
    army_list: Option<ArmyList>,
}

#[derive(Template)]
#[template(path = "army-list.html")]
struct ArmyList {
    name: String,
    points: String,
    system: String,
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
                points: parse_points(&input).to_string(),
                system: parse_system(&input).to_string(),
            })
        }
    }
}

fn parse_name(input: &String) -> &str {
    let re = Regex::new(r"^\+\+ (.*) \[").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract name]")
}

fn parse_points(input: &String) -> &str {
    let re = Regex::new(r"([\d]+)pts").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract points]")
}

fn parse_system(input: &String) -> &str {
    let re = Regex::new(r"\[([[:alpha:]]+) ").unwrap();
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .unwrap_or("[error: can't extract system]")
}

fn view(model: &Model) -> Node<Msg> {
    let rendered_list = match &model.army_list {
        Some(a) => a.render().unwrap_or_else(|err| format!("{}", err)),
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
