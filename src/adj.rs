// IMPORTS
// commons
use crate::common_enums;
use common_enums::{Gender, Number};

// wordgroups
use crate::wordgroup;
use wordgroup::{
    WordGroup
};

// strings
use crate::string;
use string::{
    take_last_grapheme,
    take_last_graphemes,
    drop_last_graphemes,
};

// adjectives
use crate::adj_enums;
use adj_enums::{AdjId};

// EXPORTS
#[derive(Clone)]
pub struct Adj {
    pub id: AdjId,
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
                    foots: self.word.foots,
                },
                None => WordGroup {
                    text: String::from([&self.word.text, "s"].join("")),
                    foots: self.word.foots,
                },
            },
            ToAgree::Form(Gender::Female, Number::Singular) => match &self.fem {
                Some(fem) => WordGroup {
                    text: String::from(fem),
                    foots: self.word.foots,
                },
                None => WordGroup {
                    text: get_feminine(&self.word.text),
                    foots: self.word.foots,
                },
            },
            ToAgree::Form(Gender::Female, Number::Plural) => match &self.fem_plur {
                Some(fem_plur) => WordGroup {
                    text: String::from(fem_plur),
                    foots: self.word.foots,
                },
                None => {
                    let fem = get_feminine(&self.word.text);
                    WordGroup {
                        text: String::from(get_plural(&fem)),
                        foots: self.word.foots,
                    }
                }
            },
        }
    }
    pub fn new(
        id: AdjId,
        masc: &str,
        foots: (u8, u8),
    ) -> Adj {
        Adj {
            id: id,
            fem: None,
            masc_plur: None,
            fem_plur: None,
            invariable: false,
            word: WordGroup {
                text: String::from(masc),
                foots: foots,
            },
        }
    }
    pub fn new_special(
        id: AdjId,
        masc: &str,
        fem: Option<String>,
        masc_plur: Option<String>,
        fem_plur: Option<String>,
        invariable: bool,
        foots: (u8, u8),
    ) -> Adj {
        Adj {
            id: id,
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

fn get_feminine(word: &str) -> String {
    let last = take_last_grapheme(word);
    let last_two = take_last_graphemes(word, 2);
    let last_three = take_last_graphemes(word, 3);
    if &last == "e" {
        String::from(word)
    } 
    else if last_three == "eux" {
        let mut new_lemme = drop_last_graphemes(word, 3);
        new_lemme.push_str("euse");
        new_lemme
    }
    else if &last_two == "er" {
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