#[macro_use]
extern crate lazy_static;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod enums;
use enums::{Article, NounCatId, Number};

mod nouns;
use nouns::{extract_lemme, get_apposition, get_with_some_article, Noun, NOUNS, NOUN_CATS};

mod adjs;
use adjs::{AdjCatHashMap, ADJ_CATS};

fn main() {
    let mut rng = thread_rng();

    fn get_with_article(article: Article, number: Number) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| get_with_some_article(article, number, noun))
    }

    fn get_with_adjective(
        adjs: &'static AdjCatHashMap,
        number: Number,
    ) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| {
            let mut rng = thread_rng();
            let rand_adj = noun
                .can_be
                .choose(&mut rng)
                .and_then(|id| adjs.get(id))
                .and_then(|v| v.choose(&mut rng))
                .and_then(|adj| Some(adj.agreed(noun.gender, number)));
            match rand_adj {
                Some(adj) => [
                    get_with_some_article(Article::Indefinite, number, noun),
                    adj.clone(),
                ]
                .join(" "),
                None => String::from("#err#adj not found"),
            }
        })
    }

    fn get_as_noun_complement(nouns: &'static [Noun; 21]) -> Box<dyn Fn(&Noun) -> String> {
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
                Some(str) => str,
                None => String::from(""),
            };
            [noun.clone(), get_apposition(&complement)].join(" ")
        })
    }

    let with_singular_adj = get_with_adjective(&ADJ_CATS, Number::Singular);
    let with_plural_adj = get_with_adjective(&ADJ_CATS, Number::Plural);
    let combinations = [
        vec![
            (
                NounCatId::Astre,
                get_with_article(Article::Indefinite, Number::Singular),
            ),
            (NounCatId::Saison, Box::new(get_apposition)),
        ],
        vec![
            (
                NounCatId::Phenomene,
                get_with_article(Article::Indefinite, Number::Plural),
            ),
            (NounCatId::Saison, Box::new(get_apposition)),
        ],
        vec![
            (NounCatId::Astre, Box::new(extract_lemme)),
            (NounCatId::Saison, Box::new(get_apposition)),
        ],
        vec![(NounCatId::Astre, with_singular_adj)],
        vec![(NounCatId::PlanteAFleur, with_plural_adj)],
        vec![(NounCatId::OrganeDePlante, get_as_noun_complement(&NOUNS))],
    ];

    for nb in 0..combinations.len() {
        //let chosen = combination.choose(&mut rng);

        let random_result: Vec<Option<String>> = match combinations.get(nb) {
            Some(choice) => choice
                .iter()
                .map(|key| {
                    NOUN_CATS
                        .get(&key.0)
                        .and_then(|cat| cat.choose(&mut rng))
                        .and_then(|id| NOUNS.iter().find(|item| &item.id == id))
                        .and_then(|noun| Some(key.1(noun)))
                })
                .collect(),
            None => vec![],
        };
        let result = random_result
            .iter()
            .map(|res_option| match res_option {
                Some(str) => str,
                None => "",
            })
            .collect::<Vec<&str>>();
        println!("{}", result.join(" "));
    }
}
