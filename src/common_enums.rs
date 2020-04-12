use crate::adj_enums::AdjId;
use crate::noun_enums::NounId;
use crate::verb_enums::VerbId;

#[derive(Copy, Clone)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Copy, Clone)]
pub enum Number {
    Plural,
    Singular,
}

#[derive(Copy, Clone)]
pub enum Article {
    None,
    Definite,
    Indefinite,
}

pub struct BlackLists {
    pub nouns: Vec<NounId>,
    pub adjs: Vec<AdjId>,
    pub verbs: Vec<VerbId>,
}

impl BlackLists {
    pub fn new_empty() -> BlackLists {
        BlackLists {
            nouns: vec![],
            adjs: vec![],
            verbs: vec![],
        }
    }
}

pub fn merge_blacklists(a: &BlackLists, b: &BlackLists) -> BlackLists {
    BlackLists {
        nouns: [&a.nouns[..], &b.nouns[..]].concat(),
        adjs: [&a.adjs[..], &b.adjs[..]].concat(),
        verbs: [&a.verbs[..], &b.verbs[..]].concat(),
    }
}
