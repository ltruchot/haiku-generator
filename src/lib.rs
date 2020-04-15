// externals
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
#[macro_use]
extern crate lazy_static;

// commons
mod common_enums;
mod random;
mod string;
mod wordgroup;
mod hashsets;

// haikus
mod haiku;
use haiku::generate_haiku;

mod combination;
mod combination_data;
use combination_data::{get_combinations, Combinations};

// nouns
mod noun;
mod noun_data;
mod noun_enums;

// adjectives
mod adj;
mod adj_data;
mod adj_enums;

// verbs
mod verb;
mod verb_data;
mod verb_enums;

#[wasm_bindgen]
pub fn generate() -> Array {
    // prepare authorized sentence combinations
    let mut combinations: Combinations = get_combinations();
    let haiku = generate_haiku(&mut combinations);

    // return haiku
    haiku
        .unwrap_or([String::from(""), String::from(""), String::from("")])
        .iter()
        .map(JsValue::from)
        .collect()
}

