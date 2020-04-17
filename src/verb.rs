// IMPORTS
// commons
use crate::common_enums;
use common_enums::Number;

// string
use crate::string;
use string::{drop_last_graphemes, take_last_graphemes};

// wordgroup
use crate::wordgroup;
use wordgroup::{add_words, check_ellision, WordGroup};

// verbs
use crate::verb_data;
use crate::verb_enums;
use verb_data::{VERBS, VERB_CATS};
use verb_enums::{VerbCatId, VerbGroup, VerbId, VerbKind};

// EXPORTS
#[derive(Clone)]
pub struct Verb {
    pub id: VerbId,
    pub group: VerbGroup,
    pub is_pronominal: bool,
    pub kind: VerbKind,
    pub word: WordGroup,
    pub first_person: Option<WordGroup>,
    pub third_person: Option<WordGroup>,
    pub sixth_person: Option<WordGroup>,
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
            },
            first_person: None,
            third_person: None,
            sixth_person: None,
        }
    }

    pub fn new_special(
        id: VerbId,
        infinitive: &str,
        foots: (u8, u8),
        is_pronominal: bool,
        kind: VerbKind,
        first_person: Option<WordGroup>,
        third_person: Option<WordGroup>,
        sixth_person: Option<WordGroup>,
    ) -> Verb {
        Verb {
            id,
            group: VerbGroup::Third,
            is_pronominal,
            kind,
            word: WordGroup {
                text: String::from(infinitive),
                foots,
            },
            first_person,
            third_person,
            sixth_person,
        }
    }

    pub fn agree(self: &Self, number: &Number) -> WordGroup {
        let root = drop_last_graphemes(&self.word.text, 2);
        let last_two = take_last_graphemes(&root, 2).clone();
        let agreed_verb = match number {
            Number::Singular => match &self.third_person {
                Some(irregular) => irregular.clone(),
                None => match self.group {
                    VerbGroup::First => WordGroup {
                        text: if last_two == "ev" {
                            [
                                String::from(drop_last_graphemes(&root, 2)),
                                String::from("ève"),
                            ]
                            .join("")
                        } else {
                            [&root, "e"].join("")
                        },
                        foots: (self.word.foots.0 - 1, self.word.foots.1 - 1),
                    },
                    VerbGroup::Second => WordGroup {
                        text: [&root, "it"].join(""),
                        foots: self.word.foots,
                    },
                    VerbGroup::Third => {
                        WordGroup::new("error#Verb::agree#third group is not implemented yet", 0)
                    }
                },
            },
            Number::Plural => match &self.sixth_person {
                Some(irregular) => irregular.clone(),
                None => match self.group {
                    VerbGroup::First => WordGroup {
                        text: if last_two == "ev" {
                            [
                                String::from(drop_last_graphemes(&root, 2)),
                                String::from("èvent"),
                            ]
                            .join("")
                        } else {
                            [&root, "ent"].join("")
                        },
                        foots: (self.word.foots.0 - 1, self.word.foots.1 - 1),
                    },
                    VerbGroup::Second => WordGroup {
                        text: [&root, "issent"].join(""),
                        foots: self.word.foots,
                    },
                    VerbGroup::Third => {
                        WordGroup::new("error#Verb::agree#third group is not implemented yet", 0)
                    }
                },
            },
        };
        if self.is_pronominal {
            as_pronominal(&agreed_verb)
        } else {
            agreed_verb.clone()
        }
    }
}

pub fn as_pronominal(agreed_verb: &WordGroup) -> WordGroup {
    let first = agreed_verb.text.chars().next();
    let has_ellision = match first {
        Some(letter) => check_ellision(&letter),
        None => false,
    };
    let pronoun = WordGroup {
        text: String::from(if has_ellision { "s'" } else { "se" }),
        foots: if has_ellision { (0, 0) } else { (1, 1) },
    };
    return add_words(&pronoun, &agreed_verb);
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
