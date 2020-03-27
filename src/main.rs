use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

mod enums;
use enums::{Gender, AdjId, Article, CategoryId, Number}; 

mod nouns;
use nouns::{Noun, get_apposition, get_with_some_article};

mod adjs;
use adjs::{Adj};

macro_rules! str_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let mut rng = thread_rng();

    type AdjHashMap = HashMap<AdjId, Vec<Adj>>;
    let mut adjs: AdjHashMap = HashMap::new();

    adjs.insert(AdjId::EnFleur, vec![Adj::new("en fleur")]);
    adjs.insert(AdjId::Sauvage, vec![Adj::new("sauvage")]);

    type CatHashMap = HashMap<CategoryId, Vec<Noun>>;

    let categories: CatHashMap = [
        (
            CategoryId::Astre,
            vec![
                Noun::new("Lune", Gender::Female, str_vec!["lumière"], vec![]),
                Noun::new("Soleil", Gender::Male, str_vec!["lumière"], vec![]),
            ],
        ),
        (
            CategoryId::Phenomene,
            vec![
                Noun::new("bruit", Gender::Male, str_vec![], vec![]),
                Noun::new("lumière", Gender::Female, str_vec![], vec![]),
                Noun::new("odeur", Gender::Female, str_vec![], vec![]),
            ],
        ),
        (
            CategoryId::Saison,
            vec![
                Noun::new("printemps", Gender::Male, str_vec!["odeur"], vec![]),
                Noun::new("été", Gender::Male, str_vec!["odeur"], vec![]),
                Noun::new("automne", Gender::Male, str_vec!["odeur"], vec![]),
                Noun::new("hiver", Gender::Male, str_vec!["odeur"], vec![]),
            ],
        ),
        (
            CategoryId::PlanteAFleur,
            vec![
                Noun::new(
                    "prunier",
                    Gender::Male,
                    str_vec!["odeur"],
                    vec![AdjId::EnFleur, AdjId::Sauvage],
                ),
                Noun::new(
                    "cerisier",
                    Gender::Male,
                    str_vec!["odeur"],
                    vec![AdjId::EnFleur, AdjId::Sauvage],
                ),
                Noun::new(
                    "oeillet",
                    Gender::Male,
                    str_vec!["odeur"],
                    vec![AdjId::EnFleur, AdjId::Sauvage],
                ),
                Noun::new(
                    "glycine",
                    Gender::Female,
                    str_vec!["odeur"],
                    vec![AdjId::EnFleur, AdjId::Sauvage],
                ),
                Noun::new(
                    "pivoine",
                    Gender::Female,
                    str_vec!["odeur"],
                    vec![AdjId::EnFleur, AdjId::Sauvage],
                ),
            ],
        ),
        (
            CategoryId::OrganeDePlante,
            vec![
                Noun::new("feuille", Gender::Male, str_vec!["bruit"], vec![]),
                Noun::new("branche", Gender::Male, str_vec!["bruit"], vec![]),
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let nouns: Vec<Noun> = categories.iter().fold(vec![], |mut acc, cat| {
        acc.extend(cat.1.clone());
        acc
    });

    fn get_with_article(article: Article, number: Number) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| get_with_some_article(article, number, noun))
    }

    fn get_with_adjective(adjs: AdjHashMap, number: Number) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| {
            let mut rng = thread_rng();
            let rand_adj = noun
                .can_be
                .choose(&mut rng)
                .and_then(|id| adjs.get(id))
                .and_then(|v| v.first());
            let adjective = String::from(match rand_adj {
                Some(adj) => &adj.lemme,
                None => "",
            });
            [
                get_with_some_article(Article::Indefinite, number, noun),
                adjective.clone(),
            ]
            .join(" ")
        })
    }

    fn get_as_noun_complement(nouns: Vec<Noun>) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |complement| {
            let mut rng = thread_rng();
            let rand_noun = complement
                .emit
                .choose(&mut rng)
                .and_then(|name| nouns.iter().find(|item| &item.lemme == name))
                .and_then(|noun| Some(get_with_some_article(Article::Indefinite, Number::Singular, &noun)));
            let noun = match rand_noun {
                Some(str) => str,
                None => String::from(""),
            };
            [noun.clone(), get_apposition(&complement)].join(" ")
        })
    }
    let combination = [
        vec![
            (CategoryId::Astre, get_with_article(Article::Definite, Number::Singular)),
            (CategoryId::Saison, Box::new(get_apposition)),
        ],
        vec![(CategoryId::PlanteAFleur, get_with_adjective(adjs, Number::Plural))],
        vec![(CategoryId::OrganeDePlante, get_as_noun_complement(nouns))],
    ];

    for nb in 0..3 {
        //let chosen = combination.choose(&mut rng);

        let random_result: Vec<Option<String>> = match combination.get(nb) {
            Some(choice) => choice
                .iter()
                .map(|key| {
                    categories
                        .get(&key.0)
                        .and_then(|cat| cat.choose(&mut rng))
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
