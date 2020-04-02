use crate::word;
use word::{WordGroup};
use crate::verb_enums;
use verb_enums::{VerbId, VerbGroup};

#[derive(Clone)]
pub struct Verb {
    pub id: VerbId,
    pub group: VerbGroup,
    pub pronominal: bool,
    pub word: WordGroup,
}

impl Verb {
    pub fn new(
        id: VerbId,
        infinitive: &str,
        foots: (u8, u8),
        group: VerbGroup,
        pronominal: bool,
    ) -> Verb {
        Verb {
            id: id,
            group: group,
            pronominal: pronominal,
            word: WordGroup {
                text: String::from(infinitive),
                foots: foots,
            }
        }
    }
}