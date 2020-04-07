// IMPORTS
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::common_enums;
use common_enums::{Number, Article};
use rand::rngs::{ThreadRng};

// verbs
use crate::verb;
use verb::{Verb};
use crate::verb_enums;
use verb_enums::{VerbCatId};
use crate::verb_data;
use verb_data::{VERBS, VERB_CATS};

// adjs
use crate::adj_enums;
use adj_enums::{AdjCatId};
use crate::adj_data;
use adj_data::{ADJS, ADJ_CATS};
use crate::adj;
use adj::{Adj};

use crate::wordgroup;
use wordgroup::{WordGroup, add_words};

// nouns
use crate::noun;
use noun::{Noun, get_apposition};
use crate::noun_enums;
use noun_enums::{NounCatId};
use crate::noun_data;
use noun_data::{StaticNouns, NOUN_CATS};

// EXPORTS
pub type Combination = Box<dyn Fn(&Noun) -> WordGroup>;
pub type Construction = Vec<(Vec<NounCatId>, Vec<Combination>)>;

pub fn get_with_verb(verb_cats: Vec<VerbCatId>, number: Number) -> Combination {
    Box::new(move |noun| {
        let mut rng = thread_rng();
        let rand_article = get_rand_article(&mut rng);
        let verb = get_rand_verb(&mut rng, &verb_cats);
        let article = noun.get_article(number, rand_article);
        let noun_with_verb = noun.with_verb(&verb, number);
        add_words(&article, &noun_with_verb, false)
    })
}

pub fn get_rand_article (rng: &mut ThreadRng) -> Article {
    [Article::Definite, Article::Indefinite]
            .choose(rng)
            .unwrap_or(&Article::Indefinite).clone()
}

pub fn get_rand_number (rng: &mut ThreadRng) -> Number {
    [Number::Singular, Number::Plural]
            .choose(rng)
            .unwrap_or(&Number::Singular).clone()
}

pub fn get_rand_verb (rng: &mut ThreadRng, verb_cats: &Vec<VerbCatId>) -> Verb {
    verb_cats
        .choose(rng)
        .and_then(|id| VERB_CATS.get(id))
        .and_then(|v| v.choose(rng))
        .and_then(|id| VERBS.iter().find(|item| &item.id == id))
        .unwrap_or(&VERBS[0]).clone()
}

pub fn get_rand_adj(rng: &mut ThreadRng, adj_cats: &Vec<AdjCatId>) -> Adj {
    adj_cats
        .choose(rng)
        .and_then(|id| ADJ_CATS.get(id))
        .and_then(|list|list.choose(rng))
        .and_then(|id| ADJS.iter().find(|item| &item.id == id))
        .unwrap_or(&ADJS[0]).clone()
}

pub fn get_with_linking_verb(verb_cats: Vec<VerbCatId>, number_opt: Option<Number>, article_opt: Option<Article>) -> Combination {
    Box::new(move |noun: &Noun| {
        let mut rng = thread_rng();
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = article_opt.unwrap_or(get_rand_article(&mut rng));
        let wgs = [
            noun.get_with_article(article_type, number),
            get_rand_verb(&mut rng, &verb_cats).agreed(number),
            get_rand_adj(&mut rng, &noun.is).agreed(noun.gender, number),
        ];
        wgs
            .iter()
            .fold(WordGroup::new_empty(), |acc, wg|add_words(&acc, wg, acc.text != ""))
    })
}

pub fn get_with_article(article: Article, number: Number) -> Combination {
    Box::new(move |noun| noun.get_with_article(article, number))
}

pub fn get_with_rand_article() -> Combination {
    let mut rng = thread_rng();
    let article = [Article::Definite, Article::Indefinite]
        .choose(&mut rng)
        .unwrap_or(&Article::Indefinite).clone();
    let number = [Number::Singular, Number::Plural]
        .choose(&mut rng)
        .unwrap_or(&Number::Singular).clone();
        get_with_article(article, number)
}

pub fn get_with_adjective(
    article: Article,
    number: Number,
) -> Combination {
    Box::new(move |noun| {
        let mut rng = thread_rng();
        let adj = get_rand_adj(&mut rng, &noun.can_be)
            .agreed(noun.gender, number);
        let wg = noun.get_with_article(article, number);
        add_words(&wg, &adj, true)
    })
}

pub fn get_as_noun_complement(nouns: &'static StaticNouns) -> Combination {
    Box::new(move |complement| {
        let mut rng = thread_rng();
        let rand_noun = complement
            .can_emit
            .choose(&mut rng)
            .and_then(|id| NOUN_CATS.get(id))
            .and_then(|cat| cat.nouns.choose(&mut rng))
            .and_then(|id| nouns.iter().find(|item| &item.id == id))
            .and_then(|noun| {
                Some(noun.get_with_article(
                    Article::Indefinite,
                    Number::Singular,
                ))
            });
        let noun = match rand_noun {
            Some(group) => group,
            None => WordGroup {
                text: String::from("#error#get_as_noun_complement#Can't find noun"),
                foots: (0, 0),
            },
        };
        let apposition = get_apposition(&complement);

        add_words(&noun, &apposition, true)
    })
}
