// externals
#[macro_use]
extern crate lazy_static;
use rand::thread_rng;
use rand::seq::SliceRandom;

// commons
mod common_enums;
mod wordgroup;
mod string;
mod random;

// haikus
mod haiku;
use haiku::{generate_haiku};

// combinations
mod combination;
mod combination_data;
use combination_data::{get_combinations, Combinations};

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

// mod exemple;
// use exemple::{get_total};


fn main() {
    let mut rng = thread_rng();
   
    // prepare authorized sentence combinations
    let mut combinations: Combinations = get_combinations();
    combinations.shuffle(&mut rng);
    match generate_haiku(&combinations) {
        Ok(haiku) => println!("Haïku:\n{}", haiku.join("\n")),
        Err(errs) => println!("{:?}", errs)
    }
    ()  
}
