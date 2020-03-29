use unicode_segmentation::UnicodeSegmentation;
use crate::enums;
use enums::{Gender, Number}; 

#[derive(Clone)]
pub struct Adj {
    pub masc: String,
    pub fem: Option<String>,
    pub masc_plur: Option<String>,
    pub fem_plur: Option<String>,
    pub invariable: bool,
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
impl Adj {
    pub fn agreed (&self, gender: Gender, number: Number) -> String {
        if self.invariable {
            return String::from(&self.masc);
        }
        enum ToAgree {
            Form(Gender, Number)
        }
        let agreement = ToAgree::Form(gender, number);
        match agreement {
            ToAgree::Form(Gender::Male, Number::Singular) => String::from(&self.masc),
            ToAgree::Form(Gender::Male, Number::Plural) => match &self.masc_plur {
                Some(masc) => String::from(masc),
                None => String::from([&self.masc, "s"].join(""))
            },
            ToAgree::Form(Gender::Female, Number::Singular) => match &self.fem {
                Some(fem) => String::from(fem),
                None => {
                  get_feminine(&self.masc)
                }
            }, 
            ToAgree::Form(Gender::Female, Number::Plural) => match &self.fem_plur {
                Some(fem_plur) => String::from(fem_plur),
                None => {
                    let fem = get_feminine(&self.masc);
                    String::from(get_plural(&fem))
                }
            }
        }
    }
    pub fn new(
        masc: &str, fem: Option<String>, 
        masc_plur: Option<String>, 
        fem_plur: Option<String>,
        invariable: bool
    ) -> Adj {
        Adj {
            masc: String::from(masc),
            fem: fem,
            masc_plur: masc_plur,
            fem_plur: fem_plur,
            invariable: invariable
        }
    }
}
