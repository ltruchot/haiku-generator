use crate::common_enums;
use common_enums::{Number};
use crate::word;
use word::{
    WordGroup,
    add_words,
    drop_last_graphemes,
    check_ellision,
};
use crate::verb_enums;
use verb_enums::{VerbId, VerbGroup};

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
                WordGroup {
                    text: match number {
                        Number::Singular => [&root, "e"].join(""),
                        Number::Plural => [&root, "ent"].join(""),
                    },
                    foots: (self.word.foots.0 - 1, self.word.foots.1)
                }
            },
            VerbGroup::Second => {
                WordGroup {
                    text: String::from(&self.word.text),
                    foots: (0, 0)
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