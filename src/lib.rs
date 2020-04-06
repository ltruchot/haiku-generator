// externals
use wasm_bindgen::prelude::*;
use js_sys::Array;
use wasm_bindgen::JsValue;
#[macro_use]
extern crate lazy_static;
use rand::thread_rng;
use rand::seq::SliceRandom;

// commons
mod common_enums;
mod wordgroup;
mod string;

// haikus
mod haiku;
use haiku::{generate_haiku};

mod combination;
mod combination_data;
use combination_data::{get_constructions, Constructions};

// nouns
mod noun;
mod noun_enums;
mod noun_data;

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
    let mut rng = thread_rng();
   
    // prepare authorized sentence constructions
    let mut constructions: Constructions = get_constructions();
    constructions.shuffle(&mut rng);
    let haiku = generate_haiku(&constructions, &mut rng);

    // return haiku
    haiku.iter().map(JsValue::from).collect()
}
