use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

mod enums;
use enums::{AdjId, Article, CategoryId, Gender, Number};

mod nouns;
use nouns::{extract_lemme, get_apposition, get_with_some_article, Noun};

mod adjs;
use adjs::Adj;

macro_rules! str_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let mut rng = thread_rng();

    type AdjHashMap = HashMap<AdjId, Vec<Adj>>;
    let adjs: AdjHashMap = [
        (AdjId::EnFleur, vec![
            Adj::new("en fleur", None, None, None, true),
        ]),
        (AdjId::Sauvage, vec![
            Adj::new("sauvage", None, None, None, false)
        ]),
        (AdjId::RelAUneSaison, vec![
            Adj::new("printanier", None, None, None, false),
            Adj::new("estival", None, None, None, false),
            Adj::new("automnal", None, None, None, false),
            Adj::new("hivernal", None, None, None, false),
        ]),
    ]
    .iter()
    .cloned()
    .collect();

    type CatHashMap = HashMap<CategoryId, Vec<Noun>>;
    let categories: CatHashMap = [
        (
            CategoryId::Astre,
            vec![
                Noun::new(
                    "Lune",
                    Gender::Female,
                    str_vec!["lumière"],
                    vec![AdjId::RelAUneSaison],
                ),
                Noun::new(
                    "Soleil",
                    Gender::Male,
                    str_vec!["lumière"],
                    vec![AdjId::RelAUneSaison],
                ),
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
                Noun::new("printemps", Gender::Male, str_vec!["odeur", "lumière"], vec![]),
                Noun::new("été", Gender::Male, str_vec!["odeur", "lumière"], vec![]),
                Noun::new("automne", Gender::Male, str_vec!["odeur", "lumière"], vec![]),
                Noun::new("hiver", Gender::Male, str_vec!["odeur", "lumière"], vec![]),
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
                Noun::new("feuille", Gender::Male, str_vec!["bruit", "odeur"], vec![]),
                Noun::new("branche", Gender::Male, str_vec!["bruit", "odeur"], vec![]),
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

    fn get_with_adjective(
        adjectives: AdjHashMap,
        number: Number,
    ) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| {
            let mut rng = thread_rng();
            let rand_adj = noun
                .can_be
                .choose(&mut rng)
                .and_then(|id| adjectives.get(id))
                .and_then(|v| v.choose(&mut rng))
                .and_then(|adj|Some(adj.agreed(noun.gender, number)));
            match rand_adj {
                Some(adj) =>            [
                    get_with_some_article(Article::Indefinite, number, noun),
                    adj.clone(),
                ]
                .join(" "),
                None => String::from("#err#adj not found")
            }
 
        })
    }

    fn get_as_noun_complement(nouns: Vec<Noun>) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |complement| {
            let mut rng = thread_rng();
            let rand_noun = complement
                .emit
                .choose(&mut rng)
                .and_then(|name| nouns.iter().find(|item| &item.lemme == name))
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

    let with_singular_adj = get_with_adjective(adjs.clone(), Number::Singular);
    let with_plural_adj = get_with_adjective(adjs.clone(), Number::Plural);
    
    let combinations = [
        vec![
            (
                CategoryId::Astre,
                get_with_article(Article::Indefinite, Number::Singular),
            ),
            (CategoryId::Saison, Box::new(get_apposition)),
        ],
        vec![
            (
                CategoryId::Phenomene,
                get_with_article(Article::Indefinite, Number::Plural),
            ),
            (CategoryId::Saison, Box::new(get_apposition)),
        ],
        vec![
            (CategoryId::Astre, Box::new(extract_lemme)),
            (CategoryId::Saison, Box::new(get_apposition)),
        ],
        vec![(
            CategoryId::Astre,
            with_singular_adj,
        )],
        vec![(
            CategoryId::PlanteAFleur,
            with_plural_adj
        )],
        vec![(CategoryId::OrganeDePlante, get_as_noun_complement(nouns))],
    ];

    for nb in 0..combinations.len() {
        //let chosen = combination.choose(&mut rng);

        let random_result: Vec<Option<String>> = match combinations.get(nb) {
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
