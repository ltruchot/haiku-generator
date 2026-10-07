// externals

// commons
mod common_enums;
mod hashsets;
mod random;
mod string;
mod wordgroup;

// haikus
mod haiku;
use haiku::generate_haiku;

// combinations
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

// mod exemple;
// use exemple::{get_total};

#[cfg(test)]
mod characterization;

fn main() {
    // prepare authorized sentence combinations
    let mut combinations: Combinations = get_combinations();
    match generate_haiku(&mut combinations) {
        Ok(haiku) => println!("Haïku:\n{}", haiku.join("\n")),
        Err(errs) => println!("{:?}", errs),
    }
    ()
}
