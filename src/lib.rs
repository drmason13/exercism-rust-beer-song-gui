// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::str::FromStr;

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
    start: TextBuffer<u32>,
    end: TextBuffer<u32>,
}

/// Stores a raw text value and the parsed result
#[derive(Default)]
struct TextBuffer<T: FromStr> {
    pub raw: String,
    pub parsed: T,
}

/// parsing the raw text value may depend on external information
/// this additional validation logic is parsed in as a closure
/// which will need to capture from its calling environment if needed
impl<T: FromStr + std::cmp::PartialEq + std::fmt::Display> TextBuffer<T> {
    /// update the raw value, this method could easily have been named set_raw
    pub fn update(&mut self, raw: String) {
        self.raw = raw;
    }

    /// overwrite both the parsed and raw values by providing a ready parsed value
    /// which is converted to a string
    pub fn overwrite(&mut self, parsed: T) {
        self.raw = parsed.to_string();
        self.parsed = parsed;
    }

    /// parse the current raw value and store it
    pub fn parse(&mut self) {
        if let Ok(parsed) = self.raw.parse() {
            self.parsed = parsed;
        }
    }

    /// parse the current raw value and then store the result of it going through a given
    /// validation function the provided closure takes a reference to the (successfully) parsed
    /// value and should return Some(T) to store T or None to make no changes
    pub fn validate<F>(&mut self, validate: F)
    where
        F: Fn(&T) -> Option<T>
    {
        if let Ok(parsed) = self.raw.parse() {
            if let Some(valid) = validate(&parsed) {
                self.parsed = valid;
            }
        }
    }

    /// returns true if the parsed value and raw value currently match
    pub fn is_valid(&self) -> bool {
        if let Ok(parsed) = self.raw.parse() {
            self.parsed == parsed
        } else {
            false
        }
    }
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
            // update the raw value
            model.start.update(raw);
            // check both start and end if their raw values have become valid (their validity is inter-dependent)
            // note the order is significant here:
            //   * update start first because it has just changed, by checking against the existing end value
            //   * then with a new start value we should check if start is now valid
            let prev_end = model.end.parsed;
            model.start.validate(|start| if start >= &prev_end { Some(std::cmp::min(*start, 99)) } else { None });
            let new_start = model.start.parsed;
            model.end.validate(|end| if end <= &new_start { Some(std::cmp::min(*end, 99)) } else { None });
        },
        Msg::UpdateEnd(raw) => {
            // update the raw value
            model.end.update(raw);

            // check both start and end if their raw values have become valid (their validity is inter-dependent)
            // note the order is significant here:
            //   * update end first because it has just changed, by checking against the existing start value
            //   * then with a new end value we should check if start is now valid
            let prev_start = model.start.parsed;
            model.end.validate(|end| if end <= &prev_start { Some(std::cmp::min(*end, 99)) } else { None });
            let new_end = model.end.parsed;
            model.start.validate(|start| if start >= &new_end { Some(std::cmp::min(*start, 99)) } else { None });
        },
        Msg::FullSong => {
            model.start.overwrite(99);
            model.end.overwrite(0);
        },
        Msg::NextVerse => {
            match model.end.parsed {
                0 => {
                    model.start.overwrite(99);
                    model.end.overwrite(99);
                },
                x => {
                    model.end.overwrite(x - 1);
                }
            };
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
                    attrs!{ At::Type => "number", At::Value => model.start.raw },
                    input_ev(Ev::Input, Msg::UpdateStart)
                ],
                input![
                    C!["to"],
                    attrs!{ At::Type => "number", At::Value => model.end.raw },
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
                sing(model.start.parsed, model.end.parsed).lines().map(|verse| li![verse]).collect::<Vec<Node<Msg>>>()
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
