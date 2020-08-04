// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use base::{Header, Footer};

use beer_song::sing;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
// buffers are used to hold the string when it isn't (yet) a valid number
// otherwise the logic made it sometimes impossible (or stupidly awkward) to change values
// buffers are rendered on the page, start and end are actually used in the calculation
// it might make sense to group them under a custom struct so we can codify their linkage
#[derive(Default)]
struct Model {
    start: u32,
    start_buffer: String,
    end: u32,
    end_buffer: String,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    UpdateStart(String),
    UpdateEnd(String),
    FullSong,
    NextVerse,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateStart(raw) => {
            if let Ok(start) = raw.parse::<u32>() {
                if start >= model.end {
                    model.start = std::cmp::min(start, 99);
                }
            } else {
                model.start = model.end;
            }
            model.start_buffer = raw;
        },
        Msg::UpdateEnd(raw) => {
            if let Ok(end) = raw.parse::<u32>() {
                if end <= model.start {
                    model.end = std::cmp::min(end, 99);
                }
            } else {
                model.end = model.start;
            }
            model.end_buffer = raw;
        },
        Msg::FullSong => {
            model.start = 99;
            model.end = 0;
            model.start_buffer = model.start.to_string();
            model.end_buffer = model.end.to_string();
        },
        Msg::NextVerse => {
            match model.end {
                0 => {
                    model.start = 99;
                    model.end = 99;
                },
                x => {
                    model.end = x - 1;
                }
            };
            model.start_buffer = model.start.to_string();
            model.end_buffer = model.end.to_string();
        },
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        Header::new("Beer Song"),
        section![
            C!["content"],
            div![
                C!["flexbox-row"],
                input![
                    C!["from"],
                    attrs!{ At::Type => "number", At::Value => model.start_buffer },
                    input_ev(Ev::Input, Msg::UpdateStart)
                ],
                input![
                    C!["to"],
                    attrs!{ At::Type => "number", At::Value => model.end_buffer },
                    input_ev(Ev::Input, Msg::UpdateEnd)
                ]
            ],
            div![
                C!["controls"],
                button![
                    C!["full-song"],
                    "Full Song",
                    ev(Ev::Click, |_| Msg::FullSong)
                ],
                button![
                    C!["next-verse"],
                    "Next Verse",
                    ev(Ev::Click, |_| Msg::NextVerse)
                ]
            ],
            ul![
                C!["song"],
                sing(model.start, model.end).lines().map(|verse| li![verse]).collect::<Vec<Node<Msg>>>()
            ],
        ]
        Footer::new("Beer Song", "Choose a range of verses of \"the beer song\" to 'sing'"),
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
