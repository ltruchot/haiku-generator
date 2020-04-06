// IMPORTS
// commons
use crate::common_enums;
use common_enums::{Number};

// string
use crate::string;
use string::{drop_last_graphemes, take_last_graphemes};

// wordgroup
use crate::wordgroup;
use wordgroup::{
    WordGroup,
    add_words,
    check_ellision,
};

// verbs
use crate::verb_enums;
use verb_enums::{VerbId, VerbGroup};



// EXPORTS
#[derive(Clone)]
pub struct Verb {
    pub id: VerbId,
    pub group: VerbGroup,
    pub is_pronominal: bool,
    pub word: WordGroup,
}

impl Verb {
    pub fn new(
        id: VerbId,
        infinitive: &str,
        foots: (u8, u8),
        group: VerbGroup,
        is_pronominal: bool,
    ) -> Verb {
        Verb {
            id: id,
            group: group,
            is_pronominal: is_pronominal,
            word: WordGroup {
                text: String::from(infinitive),
                foots: foots,
            }
        }
    }

    pub fn agreed(self: &Self, number: Number) -> WordGroup {
        let agreed_verb = match &self.group {
            VerbGroup::First => {
                let root = drop_last_graphemes(&self.word.text, 2);
                let last_two = take_last_graphemes(&root, 2).clone();
                WordGroup {
                    text: match number {                        
                        Number::Singular => {                            
                            if last_two == "ev" { 
                                [
                                    String::from(drop_last_graphemes(&root, 2)),
                                    String::from("ève")
                                ].join("")
                            } else { 
                                [&root, "e"].join("") 
                            }
                        },
                        Number::Plural => {
                            if last_two == "ev" { 
                                [
                                    String::from(drop_last_graphemes(&root, 2)),
                                    String::from("ève")
                                ].join("èvent")
                            } else { 
                                [&root, "ent"].join("") 
                            }
                        }
                    },
                    foots: (self.word.foots.0 - 1, self.word.foots.1)
                }
            },
            VerbGroup::Second => {
                let root = drop_last_graphemes(&self.word.text, 2);
                WordGroup {
                    text: match number {
                        Number::Singular => [&root, "it"].join(""),
                        Number::Plural => [&root, "issent"].join(""),
                    },
                    foots: (self.word.foots.0, self.word.foots.1 + 1)
                }
            },
        };
        if self.is_pronominal {
            let first = agreed_verb.text.chars().next();
            let has_ellision = match first {
                Some(letter) => check_ellision(&letter),
                None => false
            };
            let pronoun = WordGroup {
                text: String::from(if has_ellision { "s'" } else { "se "}),
                foots: if has_ellision { (0, 0) } else { (1, 1) }
            };
            return add_words(&pronoun, &agreed_verb, false);
        }
        agreed_verb
    }
}