// IMPORTS
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::common_enums;
use common_enums::{Article,  Number};

use crate::word;
use word::{WordGroup, add_words};
use crate::noun;
use noun::{Noun, get_with_some_article, get_apposition};
use crate::adj_data;
use adj_data::{AdjCatHashMap, ADJS};
use crate::noun_data;
use noun_data::{StaticNouns, NOUN_CATS};

// EXPORTS
pub type Combination = Box<dyn Fn(&Noun) -> WordGroup>;

pub fn get_with_article(article: Article, number: Number) -> Combination {
    Box::new(move |noun| get_with_some_article(article, number, noun))
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
    adjs: &'static AdjCatHashMap,
    article: Article,
    number: Number,
) -> Combination {
    Box::new(move |noun| {
        let mut rng = thread_rng();
        let rand_adj = noun
            .can_be
            .choose(&mut rng)
            .and_then(|id| adjs.get(id))
            .and_then(|v| v.choose(&mut rng))
            .and_then(|id| ADJS.iter().find(|item| &item.id == id))
            .and_then(|adj| Some(adj.agreed(noun.gender, number)));

        let wg = match rand_adj {
            Some(adj) => {
                let wg = get_with_some_article(article, number, noun);
                add_words(
                    WordGroup {
                        text: [&wg.text, " "].join(""),
                        foots: wg.foots,
                    },
                    adj.clone(),
                )
            }
            None => WordGroup {
                text: String::from("#err#adj not found"),
                foots: (0, 0),
            },
        };
        wg
    })
}

pub fn get_as_noun_complement(nouns: &'static StaticNouns) -> Combination {
    Box::new(move |complement| {
        let mut rng = thread_rng();
        let rand_noun = complement
            .emit
            .choose(&mut rng)
            .and_then(|id| NOUN_CATS.get(id))
            .and_then(|v| v.choose(&mut rng))
            .and_then(|id| nouns.iter().find(|item| &item.id == id))
            .and_then(|noun| {
                Some(get_with_some_article(
                    Article::Indefinite,
                    Number::Singular,
                    &noun,
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
        add_words(
            noun,
            WordGroup {
                text: [" ", &apposition.text].join(""),
                foots: apposition.foots,
            },
        )
    })
}
