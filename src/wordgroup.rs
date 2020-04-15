// IMPORTS
// externals
use unicode_segmentation::UnicodeSegmentation;
use crate::string::{take_last_graphemes, take_last_grapheme};

#[derive(Clone, PartialEq, Debug)]
pub struct WordGroup {
    pub text: String,
    pub foots: (u8, u8), // min / max
}

impl WordGroup {
    pub fn new(text: &str, foots_base: u8) -> WordGroup {
        WordGroup {
            text: String::from(text),
            foots: (foots_base, foots_base)
        }
    }
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
    .fold(WordGroup::new_empty(), |acc, wg|add_words(&acc, wg))
}

pub fn check_ellision (letter: &char) -> bool {
    let ellisions = ['a', 'e', 'i', 'o', 'u', 'é', 'h'];
    ellisions.contains(letter)
}

pub fn add_words (a: &WordGroup, b: &WordGroup) -> WordGroup {
    // println!("add {} {}", a.text, b.text);
    // "e" muet en français
    // shortcircuit for des, de, les, le , se
    let last = take_last_grapheme(&a.text);
    let three_last = take_last_graphemes(&a.text, 3);
    let shortcircuit = three_last == " le" ||
    three_last == " de" || three_last == " se" ||
     a.text.graphemes(true).count() <= 3 && (
        a.text == "le" ||
        a.text == "se" ||
        a.text == "de" ||
        a.text == "les" ||
        a.text == "des"
    );
    // println!("{}", shortcircuit);
    let with_space = last != "'" && &a.text != "";

    let should_amend_foots = if !shortcircuit {
        let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s","t", "v", "w", "x", "z"];
        let two_last = take_last_graphemes(&a.text, 2);
        // println!("lasts: {} {}", last, two_last);
        let penultimate_opt = if last == "e"  {
            a.text.trim().graphemes(true).rev().nth(1)
        } else if two_last == "es" {
            a.text.trim().graphemes(true).rev().nth(2)
        } else {
            None
        };
        match penultimate_opt{
            Some(prev) => {
                // println!("penultieme {}", prev);
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
    // println!("add {} {} {}", a.text, b.text, should_amend_foots);
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

#[cfg(test)]
mod tests {
    #[test]
    fn addword_works() {
        let arti = super::WordGroup::new("le", 1);
        let nou1 = super::WordGroup::new("bruissement", 3);
        let appo = super::WordGroup::new("de", 1);
        let nou2 = super::WordGroup::new("feuille", 1);
        let res1 = super::WordGroup::new("le bruissement", 4);
        let res2 = super::WordGroup::new("le bruissement de", 5);
        let res3 = super::WordGroup::new("le bruissement de feuille", 6);
        assert_eq!(
            super::add_words(&arti, &nou1), 
            res1                
        );
        assert_eq!(
            super::add_words(&res1, &appo), 
            res2              
        );
        assert_eq!(
            super::add_words(&res2, &nou2), 
            res3              
        );
    }
}
