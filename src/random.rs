// IMPORTS
use std::collections::HashSet;
use crate::common_enums::Article;
use crate::common_enums::Number;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

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
use adj_data::{ADJS, ADJ_CATS};
use adj_enums::{AdjCatId, AdjId};

// nouns
use crate::noun;
use crate::noun_data;
use crate::noun_enums;
use noun::Noun;
use noun_data::{NounCategory, NOUNS, NOUN_CATS};
use noun_enums::{NounCatId, NounId};

pub fn get_rand_noun_cat(
    rng: &mut ThreadRng,
    cats: &Vec<NounCatId>,
) -> Result<NounCategory, String> {
    cats.choose(rng)
        .and_then(|id| NOUN_CATS.get(id))
        .and_then(|cat| Some(cat.clone()))
        .ok_or(String::from("err#get_rand_cat_noun#Category not found"))
}

pub fn get_rand_adj_cat(rng: &mut ThreadRng, cats: &Vec<AdjCatId>) -> Result<Vec<AdjId>, String> {
    cats.choose(rng)
        .and_then(|id| ADJ_CATS.get(id))
        .and_then(|cat| Some(cat.clone()))
        .ok_or(String::from("err#get_rand_adj_cat#Category not found"))
}

pub fn get_rand_adj(rng: &mut ThreadRng, adj_cat: &Vec<AdjId>, blacklist: &HashSet<AdjId>) -> Result<Adj, String> {
    adj_cat
        .iter()
        .filter(|item| !blacklist.contains(&item))
        .cloned()
        .collect::<Vec<AdjId>>()
        .choose(rng)
        .and_then(|id| ADJS.iter().find(|item| &item.id == id))
        .and_then(|cat| Some(cat.clone()))
        .ok_or(String::from("err#get_rand_adj#Adjective not found"))
}

pub fn get_rand_noun(
    rng: &mut ThreadRng,
    nouns: &Vec<NounId>,
    blacklist: &HashSet<NounId>,
) -> Result<Noun, String> {
    nouns
        .iter()
        .filter(|item| !blacklist.contains(&item))
        .cloned()
        .collect::<Vec<NounId>>()
        .choose(rng)
        .and_then(|id| NOUNS.iter().find(|item| item.id == *id))
        .and_then(|item| Some(item.clone()))
        .ok_or(String::from("err#get_rand_noun#Noun not found"))
}

pub fn get_rand_verb(rng: &mut ThreadRng, verbs: &Vec<VerbId>, blacklist: &HashSet<VerbId>) -> Result<Verb, String> {
        verbs
        .iter()
        .filter(|item| !blacklist.contains(&item))
        .cloned()
        .collect::<Vec<VerbId>>()
        .choose(rng)
        .and_then(|id| VERBS.iter().find(|item| &item.id == id))
        .and_then(|item| Some(item.clone()))
        .ok_or(String::from("err#get_rand_verb#Verb not found"))
}

pub fn get_rand_article(rng: &mut ThreadRng, articles_opt: Option<&Vec<Article>>) -> Article {
    articles_opt
        .as_ref()
        .unwrap_or(&&vec![Article::None, Article::Definite, Article::Indefinite])
        .choose(rng)
        .unwrap_or(&Article::Indefinite)
        .clone()
}

pub fn get_rand_number(rng: &mut ThreadRng) -> Number {
    [Number::Singular, Number::Plural]
        .choose(rng)
        .unwrap_or(&Number::Singular)
        .clone()
}
