use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use crate::enums;
use crate::words;
use enums::{AdjCatId, Gender, Number};
use words::WordGroup;

#[derive(Clone)]
pub struct Adj {
    pub fem: Option<String>,
    pub masc_plur: Option<String>,
    pub fem_plur: Option<String>,
    pub invariable: bool,
    pub word: WordGroup,
}
impl Adj {
    pub fn agreed(&self, gender: Gender, number: Number) -> WordGroup {
        if self.invariable {
            return self.word.clone();
        }
        enum ToAgree {
            Form(Gender, Number),
        }
        let agreement = ToAgree::Form(gender, number);
        match agreement {
            ToAgree::Form(Gender::Male, Number::Singular) => self.word.clone(),
            ToAgree::Form(Gender::Male, Number::Plural) => match &self.masc_plur {
                Some(masc) => WordGroup { 
                    text: String::from(masc), 
                    foots: self.word.foots 
                },
                None => WordGroup { 
                    text: String::from([&self.word.text, "s"].join("")), 
                    foots: self.word.foots 
                }
            },
            ToAgree::Form(Gender::Female, Number::Singular) => match &self.fem {
                Some(fem) => WordGroup { 
                    text: String::from(fem), 
                    foots: self.word.foots 
                },
                None => WordGroup { 
                    text: get_feminine(&self.word.text), 
                    foots: self.word.foots 
                }
            },
            ToAgree::Form(Gender::Female, Number::Plural) => match &self.fem_plur {
                Some(fem_plur) => WordGroup { 
                    text: String::from(fem_plur), 
                    foots: self.word.foots 
                },
                None => {
                    let fem = get_feminine(&self.word.text);
                    WordGroup { 
                        text: String::from(get_plural(&fem)), 
                        foots: self.word.foots 
                    }                    
                }
            },
        }
    }
    pub fn new(
        masc: &str,
        fem: Option<String>,
        masc_plur: Option<String>,
        fem_plur: Option<String>,
        invariable: bool,
        foots: (u8, u8),
    ) -> Adj {
        Adj {
            fem: fem,
            masc_plur: masc_plur,
            fem_plur: fem_plur,
            invariable: invariable,
            word: WordGroup {
                text: String::from(masc),
                foots: foots,
            },
        }
    }
}

fn take_last_grapheme(word: &str) -> String {
    let s = String::from(word);
    // check last
    let last = s.graphemes(true).last();
    String::from(match last {
        Some(letter) => letter,
        None => "",
    })
}

fn take_last_graphemes(word: &str, n: usize) -> String {
    let last_two_rev = word
        .graphemes(true)
        .rev()
        .take(n)
        .collect::<Vec<&str>>()
        .join("");
    String::from(
        last_two_rev
            .graphemes(true)
            .rev()
            .collect::<Vec<&str>>()
            .join(""),
    )
}

fn drop_last_graphemes(word: &str, n: usize) -> String {
    let rev = word
        .graphemes(true)
        .rev()
        .skip(n)
        .collect::<Vec<&str>>()
        .join("");
    String::from(rev.graphemes(true).rev().collect::<Vec<&str>>().join(""))
}
fn get_feminine(word: &str) -> String {
    let last = take_last_grapheme(word);
    let last_two = take_last_graphemes(word, 2);
    if &last == "e" {
        String::from(word)
    } else if &last_two == "er" {
        let mut new_lemme = drop_last_graphemes(word, 2);
        new_lemme.push_str("ère");
        new_lemme
    } else if &last_two == "et" {
        let mut new_lemme = drop_last_graphemes(word, 2);
        new_lemme.push_str("ette");
        new_lemme
    } else {
        String::from([word, "e"].join(""))
    }
}

fn get_plural(word: &str) -> String {
    let last = take_last_grapheme(word);
    let last_two = take_last_graphemes(word, 2);
    if &last == "s" || &last == "x" {
        String::from(word)
    } else if &last_two == "al" {
        let mut new_lemme = drop_last_graphemes(word, 2);
        new_lemme.push_str("aux");
        new_lemme
    } else {
        String::from([word, "s"].join(""))
    }
}

pub type AdjCatHashMap = HashMap<AdjCatId, Vec<Adj>>;
lazy_static! {
    pub static ref ADJ_CATS: AdjCatHashMap = [
        (
            AdjCatId::EnFleur,
            vec![Adj::new("en fleur", None, None, None, true, (2, 2))],
        ),
        (
            AdjCatId::Sauvage,
            vec![Adj::new("sauvage", None, None, None, false, (2, 3))],
        ),
        (
            AdjCatId::RelAUneSaison,
            vec![
                Adj::new("printanier", None, None, None, false, (3, 4)),
                Adj::new("estival", None, None, None, false, (3, 3)),
                Adj::new("automnal", None, None, None, false, (3, 3)),
                Adj::new("hivernal", None, None, None, false, (3, 3)),
            ],
        ),
        (
            AdjCatId::Couleur,
            vec![
                Adj::new("violet", None, None, None, false, (2, 3)),
                Adj::new("orange", None, None, None, false, (2, 3)),
                Adj::new("brun", None, None, None, false, (2, 3)),
                Adj::new("doré", None, None, None, false, (3, 3)),
                Adj::new("argenté", None, None, None, false, (3, 3)),
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
