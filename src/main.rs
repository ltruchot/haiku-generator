use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

macro_rules! str_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[derive(Copy, Clone)]
enum Gender {
    Female,
    Male,
}

#[derive(Copy, Clone)]
enum Article {
    Definite,
    Indefinite,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum AdjId {
    EnFleur,
    Sauvage,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum CategoryId {
    Astre,
    Phenomene,
    Saison,
    PlanteAFleur,
    OrganeDePlante,
}

struct Adj {
    lemme: String,
}

impl Adj {
    fn new(lemme: &str) -> Adj {
        Adj {
            lemme: String::from(lemme),
        }
    }
}

#[derive(Clone)]
struct Noun {
    lemme: String,
    gender: Gender,
    emit: Vec<String>,
    can_be: Vec<AdjId>,
}

impl Noun {
    fn new(lemme: &str, gender: Gender, emissions: Vec<String>, adjs: Vec<AdjId>) -> Noun {
        Noun {
            lemme: String::from(lemme),
            gender: gender,
            emit: emissions,
            can_be: adjs,
        }
    }
}

fn get_with_some_article(article: Article, noun: &Noun) -> String {
    let article = match noun.gender {
        Gender::Male => match article {
            Article::Definite => "le",
            Article::Indefinite => "un",
        },
        Gender::Female => match article {
            Article::Definite => "la",
            Article::Indefinite => "une",
        },
    };
    let mut word = String::new();
    word.push_str(article);
    word.push(' ');
    word.push_str(&noun.lemme);
    word
}

fn get_apposition(noun: &Noun) -> String {
    let first = noun.lemme.chars().next();
    match first {
        Some(letter) => {
            let ellisions = ['a', 'é', 'h'];
            [
                if ellisions.contains(&letter) {
                    "d'"
                } else {
                    "de "
                },
                &noun.lemme,
            ]
            .join("")
        }
        None => String::from(""),
    }
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

    fn get_with_article(article: Article) -> Box<dyn Fn(&Noun) -> String> {
        Box::new(move |noun| get_with_some_article(article, noun))
    }

    fn get_with_adjective(adjs: AdjHashMap) -> Box<dyn Fn(&Noun) -> String> {
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
                get_with_some_article(Article::Indefinite, noun),
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
                .and_then(|noun| Some(get_with_some_article(Article::Indefinite, &noun)));
            let noun = match rand_noun {
                Some(str) => str,
                None => String::from(""),
            };
            [noun.clone(), get_apposition(&complement)].join(" ")
        })
    }
    let combination = [
        vec![
            (CategoryId::Astre, get_with_article(Article::Definite)),
            (CategoryId::Saison, Box::new(get_apposition)),
        ],
        vec![(CategoryId::PlanteAFleur, get_with_adjective(adjs))],
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
