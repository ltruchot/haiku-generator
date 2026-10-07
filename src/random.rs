// IMPORTS
use crate::common_enums::Article;
use crate::common_enums::Number;
use rand::RngExt;
use rand::rngs::ThreadRng;
use std::collections::HashSet;

// verbs
use crate::verb;
use crate::verb_data;
use crate::verb_enums;
use verb::Verb;
use verb_data::VERBS;
use verb_enums::VerbId;

// adjs
use crate::adj;
use crate::adj_data;
use crate::adj_enums;
use adj::Adj;
use adj_data::{ADJ_CATS, ADJS};
use adj_enums::{AdjCatId, AdjId};

// nouns
use crate::noun;
use crate::noun_data;
use crate::noun_enums;
use noun::Noun;
use noun_data::{NOUN_CATS, NOUNS, NounCategory};
use noun_enums::{NounCatId, NounId};

// The only seam between the crate and `rand`: every random decision is the
// choice of an index in `0..len`.
pub trait Chooser {
    /// `len` is at least 1; the result is in `0..len`.
    fn index(&mut self, len: usize) -> usize;
}

pub struct RandChooser(ThreadRng);

impl RandChooser {
    pub fn new() -> RandChooser {
        RandChooser(rand::rng())
    }
}

impl Chooser for RandChooser {
    fn index(&mut self, len: usize) -> usize {
        self.0.random_range(0..len)
    }
}

/// One item of `items`, or `None` (without calling `index`) when it is empty.
pub fn choose<'a, T>(rng: &mut dyn Chooser, items: &'a [T]) -> Option<&'a T> {
    if items.is_empty() {
        None
    } else {
        items.get(rng.index(items.len()))
    }
}

/// Fisher-Yates: `len - 1` choices, among `len`, `len - 1`, ... 2 items.
pub fn shuffle<T>(rng: &mut dyn Chooser, items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = rng.index(i + 1);
        items.swap(i, j);
    }
}

pub fn get_rand_noun_cat(
    rng: &mut dyn Chooser,
    cats: &[NounCatId],
) -> Result<NounCategory, String> {
    choose(rng, cats)
        .and_then(|id| NOUN_CATS.get(id))
        .cloned()
        .ok_or(String::from("err#get_rand_cat_noun#Category not found"))
}

pub fn get_rand_adj_cat(rng: &mut dyn Chooser, cats: &[AdjCatId]) -> Result<Vec<AdjId>, String> {
    choose(rng, cats)
        .and_then(|id| ADJ_CATS.get(id))
        .cloned()
        .ok_or(String::from("err#get_rand_adj_cat#Category not found"))
}

pub fn get_rand_adj(
    rng: &mut dyn Chooser,
    adj_cat: &[AdjId],
    blacklist: &HashSet<AdjId>,
) -> Result<Adj, String> {
    choose(
        rng,
        &adj_cat
            .iter()
            .filter(|item| !blacklist.contains(item))
            .cloned()
            .collect::<Vec<AdjId>>(),
    )
    .and_then(|id| ADJS.iter().find(|item| &item.id == id))
    .cloned()
    .ok_or(String::from("err#get_rand_adj#Adjective not found"))
}

pub fn get_rand_noun(
    rng: &mut dyn Chooser,
    nouns: &[NounId],
    blacklist: &HashSet<NounId>,
) -> Result<Noun, String> {
    choose(
        rng,
        &nouns
            .iter()
            .filter(|item| !blacklist.contains(item))
            .cloned()
            .collect::<Vec<NounId>>(),
    )
    .and_then(|id| NOUNS.iter().find(|item| item.id == *id))
    .cloned()
    .ok_or(String::from("err#get_rand_noun#Noun not found"))
}

pub fn get_rand_verb(
    rng: &mut dyn Chooser,
    verbs: &[VerbId],
    blacklist: &HashSet<VerbId>,
) -> Result<Verb, String> {
    choose(
        rng,
        &verbs
            .iter()
            .filter(|item| !blacklist.contains(item))
            .cloned()
            .collect::<Vec<VerbId>>(),
    )
    .and_then(|id| VERBS.iter().find(|item| &item.id == id))
    .cloned()
    .ok_or(String::from("err#get_rand_verb#Verb not found"))
}

pub fn get_rand_article(rng: &mut dyn Chooser, articles_opt: Option<&Vec<Article>>) -> Article {
    *choose(
        rng,
        articles_opt.as_ref().unwrap_or(&&vec![
            Article::None,
            Article::Definite,
            Article::Indefinite,
        ]),
    )
    .unwrap_or(&Article::Indefinite)
}

pub fn get_rand_number(rng: &mut dyn Chooser) -> Number {
    *choose(rng, &[Number::Singular, Number::Plural]).unwrap_or(&Number::Singular)
}
