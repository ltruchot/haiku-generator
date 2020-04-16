use crate::hashsets::merge_hashsets;
use std::collections::HashSet;
use std::iter::FromIterator;
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

#[derive(Clone)]
pub struct BlackLists {
    pub nouns: HashSet<NounId>,
    pub adjs: HashSet<AdjId>,
    pub verbs: HashSet<VerbId>,
}

impl BlackLists {
    pub fn new_empty() -> BlackLists {
        BlackLists {
            nouns: HashSet::new(),
            adjs: HashSet::new(),
            verbs: HashSet::new(),
        }
    }
}

pub fn merge_blacklists(a: &BlackLists, b: &BlackLists) -> BlackLists {
    BlackLists {
        nouns: merge_hashsets(&a.nouns, &b.nouns),
        adjs: merge_hashsets(&a.adjs, &b.adjs),
        verbs: merge_hashsets(&a.verbs, &b.verbs),
    }
}

pub fn adjust_blacklist (bl: BlackLists) -> BlackLists {
    // haiku should contains only one mention of a season
    let saisons = [NounId::Printemps, NounId::Ete, NounId::Automne, NounId::Hiver];
    let saison_adjs = [AdjId::Printanier, AdjId::Estival, AdjId::Automnal, AdjId::Hivernal];
    let has_saison = saisons.iter().any(|saison|bl.nouns.contains(saison));
    let has_saison_adj = saison_adjs.iter().any(|saison_adj|bl.adjs.contains(saison_adj));
    if has_saison || has_saison_adj {
        let saison_bl = BlackLists {
            nouns: HashSet::from_iter(saisons.iter().cloned()),
            adjs: HashSet::from_iter(saison_adjs.iter().cloned()),
            verbs: HashSet::new()
        };
        merge_blacklists(&bl, &saison_bl)
    } else {
        bl
    }
}
