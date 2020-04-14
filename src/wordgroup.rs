// IMPORTS
// externals
use unicode_segmentation::UnicodeSegmentation;
use crate::string::{take_last_graphemes, take_last_grapheme};

#[derive(Clone)]
pub struct WordGroup {
    pub text: String,
    pub foots: (u8, u8), // min / max
}

impl WordGroup {
    pub fn new_empty() -> WordGroup {
        WordGroup {
            text: String::from(""),
            foots: (0, 0),
        }
    }
}

pub fn fold_wordgroups (wgs: Vec<WordGroup>) -> WordGroup {
    wgs
    .iter()
    .fold(WordGroup::new_empty(), |acc, wg|add_words(&acc, wg, acc.text != ""))
}

pub fn check_ellision (letter: &char) -> bool {
    let ellisions = ['a', 'e', 'i', 'o', 'u', 'é', 'h'];
    ellisions.contains(letter)
}

pub fn add_words (a: &WordGroup, b: &WordGroup, with_space: bool) -> WordGroup {
    // println!("add {} {}", a.text, b.text);
    // "e" muet en français
    let should_amend_foots = if a.text.trim().graphemes(true).count() > 2 && a.text.trim() != "les" {
        let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s","t", "v", "w", "x", "z"];
        let last = take_last_grapheme(&a.text.trim());
        let two_last = take_last_graphemes(&a.text.trim(), 2);
        println!("lasts: {} {}", last, two_last);
        let penultimate_opt = if last == "e"  {
            a.text.trim().graphemes(true).rev().nth(1)
        } else if two_last == "es" {
            a.text.trim().graphemes(true).rev().nth(2)
        } else {
            None
        };
        match penultimate_opt{
            Some(prev) => {
                println!("penultieme {}", prev);
                consonants.contains(&prev) && ({
                    match b.text.trim().graphemes(true).nth(0) {
                        Some(letter) => {
                            // println!("next is {}", letter);
                            if two_last == "es" { true } else { consonants.contains(&letter) }
                        }
                        None => false,
                    }
                })
            },
            None => false
        }
    } else {
        false
    };
    println!("add {} {} {}", a.text, b.text, should_amend_foots);
    // combination
    WordGroup {
        text: [
            String::from(&a.text), 
            String::from(&b.text)
        ].join(if with_space {" "} else {""}),
        foots: {
            let modifier = if should_amend_foots {
                // println!("should amend {}", a.text);
                1
            } else { 0 };
            (a.foots.0 + b.foots.0 + modifier , a.foots.1 + b.foots.1 + modifier)
        }
    }
}

