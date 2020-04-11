use crate::verb_enums::VerbId;
use crate::adj_enums::AdjId;
use crate::noun_enums::NounId;

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
    pub fn new_empty () -> BlackLists {
        BlackLists {
            nouns: vec![],
            adjs: vec![],
            verbs: vec![],
        }
    }
}

