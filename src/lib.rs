#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        army: ("".to_string()),
    }
}

struct Model {
    army: String,
}

#[derive(Clone)]
enum Msg {
    ArmyUpdated(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArmyUpdated(input) => model.army = input,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        textarea![
            C!["paste-area"],
            attrs! {At::Rows => 10},
            input_ev(Ev::Change, Msg::ArmyUpdated),
            input_ev(Ev::KeyUp, Msg::ArmyUpdated)
        ],
        p!(model.army.clone())
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
