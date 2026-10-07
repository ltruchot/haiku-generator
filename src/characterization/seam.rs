// The ONLY place of the characterization tests that touches `rand`.
//
// Every production function taking `&mut ThreadRng` is called through one of
// the wrappers below, so a later rand upgrade (or the replacement of
// `ThreadRng` by a crate-local trait) is a change to this file only.
// `generate_haiku` builds its own generator and needs no wrapper.

use std::collections::HashSet;

use rand::rngs::ThreadRng;

use crate::adj::Adj;
use crate::adj_enums::{AdjCatId, AdjId};
use crate::combination::Combination;
use crate::common_enums::{Article, BlackLists, Number};
use crate::noun::Noun;
use crate::noun_data::NounCategory;
use crate::noun_enums::{NounCatId, NounId};
use crate::random;
use crate::verb::Verb;
use crate::verb_enums::VerbId;
use crate::wordgroup::WordGroup;

fn rng() -> ThreadRng {
    rand::thread_rng()
}

/// One draw of a sentence pattern.
pub fn draw(
    pattern: &Combination,
    blacklists: &BlackLists,
) -> Result<(WordGroup, BlackLists), Vec<String>> {
    pattern(&mut rng(), blacklists)
}

pub fn rand_noun_cat(cats: &Vec<NounCatId>) -> Result<NounCategory, String> {
    random::get_rand_noun_cat(&mut rng(), cats)
}

pub fn rand_adj_cat(cats: &Vec<AdjCatId>) -> Result<Vec<AdjId>, String> {
    random::get_rand_adj_cat(&mut rng(), cats)
}

pub fn rand_noun(nouns: &Vec<NounId>, blacklist: &HashSet<NounId>) -> Result<Noun, String> {
    random::get_rand_noun(&mut rng(), nouns, blacklist)
}

pub fn rand_adj(adjs: &Vec<AdjId>, blacklist: &HashSet<AdjId>) -> Result<Adj, String> {
    random::get_rand_adj(&mut rng(), adjs, blacklist)
}

pub fn rand_verb(verbs: &Vec<VerbId>, blacklist: &HashSet<VerbId>) -> Result<Verb, String> {
    random::get_rand_verb(&mut rng(), verbs, blacklist)
}

pub fn rand_article(articles: Option<&Vec<Article>>) -> Article {
    random::get_rand_article(&mut rng(), articles)
}

pub fn rand_number() -> Number {
    random::get_rand_number(&mut rng())
}
