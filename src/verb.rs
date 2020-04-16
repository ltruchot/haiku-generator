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
use verb_enums::{VerbId, VerbGroup, VerbCatId, VerbKind};
use crate::verb_data;
use verb_data::{VERB_CATS, VERBS};

// EXPORTS
#[derive(Clone)]
pub struct Verb {
    pub id: VerbId,
    pub group: VerbGroup,
    pub is_pronominal: bool,
    pub kind: VerbKind,
    pub word: WordGroup,
}

impl Verb {
    pub fn new(
        id: VerbId,
        infinitive: &str,
        foots: (u8, u8),
        group: VerbGroup,
        is_pronominal: bool,
        kind: VerbKind,
    ) -> Verb {
        Verb {
            id,
            group,
            is_pronominal,
            kind,
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
                                    String::from("èvent")
                                ].join("")
                            } else { 
                                [&root, "ent"].join("") 
                            }
                        }
                    },
                    foots: (self.word.foots.0 - 1, self.word.foots.1 - 1)
                }
            },
            VerbGroup::Second => {
                let root = drop_last_graphemes(&self.word.text, 2);
                WordGroup {
                    text: match number {
                        Number::Singular => [&root, "it"].join(""),
                        Number::Plural => [&root, "issent"].join(""),
                    },
                    foots: (self.word.foots.0, self.word.foots.1)
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
                text: String::from(if has_ellision { "s'" } else { "se"}),
                foots: if has_ellision { (0, 0) } else { (1, 1) }
            };
            return add_words(&pronoun, &agreed_verb);
        }
       agreed_verb
    }
}

pub fn get_verb_cat(id: &VerbCatId) -> Result<Vec<VerbId>, String> {
    VERB_CATS
        .get(id)
        .and_then(|cat| Some(cat.clone()))
        .ok_or(String::from("err#get_verb_cat#Can't fin category"))
}

pub fn get_verb(id: VerbId) -> Result<Verb, String> {
    VERBS
        .iter()
        .find(|verb| id == verb.id)
        .and_then(|cat| Some(cat.clone()))
        .ok_or(String::from("err#get_verb_cat#Can't fin category"))
}