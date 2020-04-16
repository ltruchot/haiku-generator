// IMPORTS
// commons
use crate::common_enums;
use common_enums::{Gender, Number};

// wordgroups
use crate::wordgroup;
use wordgroup::{WordGroup, get_plural};

// strings
use crate::string;
use string::{drop_last_graphemes, take_last_grapheme, take_last_graphemes};

// adjectives
use crate::adj_enums;
use adj_enums::AdjId;

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

        match ToAgree::Form(gender, number) {
            ToAgree::Form(Gender::Male, Number::Singular) => self.word.clone(),
            ToAgree::Form(Gender::Male, Number::Plural) => match &self.masc_plur {
                Some(masc) => WordGroup {
                    text: String::from(masc),
                    foots: self.word.foots,
                },
                None => get_plural(&self.word)
            },
            ToAgree::Form(Gender::Female, Number::Singular) => match &self.fem {
                Some(fem) => WordGroup {
                    text: String::from(fem),
                    foots: self.word.foots,
                },
                None => get_feminine(&self.word),
            },
            ToAgree::Form(Gender::Female, Number::Plural) => match &self.fem_plur {
                Some(fem_plur) => WordGroup {
                    text: String::from(fem_plur),
                    foots: self.word.foots,
                },
                None => {
                    get_plural(&get_feminine(&self.word))
                }
            },
        }
    }
    pub fn new(id: AdjId, masc: &str, foots: (u8, u8)) -> Adj {
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

fn get_feminine(wg: &WordGroup) -> WordGroup {
    let last = take_last_grapheme(&wg.text);
    let last_two = take_last_graphemes(&wg.text, 2);
    let last_three = take_last_graphemes(&wg.text, 3);
    if &last == "e" {
        wg.clone()
    } else if last_three == "eux" || last_three == "eur" {
        WordGroup {
            text: [&drop_last_graphemes(&wg.text, 3), "euse"].join(""),
            foots: wg.foots
        }
    } else if &last_two == "er" {
        WordGroup {
            text: [&drop_last_graphemes(&wg.text, 2), "ère"].join(""),
            foots: wg.foots
        }
    } else if &last_two == "et" {
        WordGroup {
            text: [&drop_last_graphemes(&wg.text, 2), "ette"].join(""),
            foots: wg.foots
        }
    } else {
        WordGroup {
            text: [&wg.text, "e"].join(""),
            foots: wg.foots
        }
    }
}
