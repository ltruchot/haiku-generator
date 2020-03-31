// externals
#[macro_use]
extern crate lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;

// commons
mod common_enums;
use common_enums::{Article, Number};
mod haikus;
use haikus::check_haiku_form;
mod word;
use word::{add_words, WordGroup};

// nouns
mod noun;
use noun::{extract_wordgroup, get_apposition, get_with_some_article, Noun};
mod noun_enums;
use noun_enums::{NounCatId};
mod noun_data;
use noun_data::{StaticNouns, NOUNS, NOUN_CATS};

// adjectives
mod adj;
use adj::{AdjCatHashMap, ADJ_CATS};
mod adj_enums;
mod adj_data;


fn main() {
    let mut rng = thread_rng();

    fn get_with_article(article: Article, number: Number) -> Box<dyn Fn(&Noun) -> WordGroup> {
        Box::new(move |noun| get_with_some_article(article, number, noun))
    }

    fn get_with_adjective(
        adjs: &'static AdjCatHashMap,
        article: Article,
        number: Number,
    ) -> Box<dyn Fn(&Noun) -> WordGroup> {
        Box::new(move |noun| {
            let mut rng = thread_rng();
            let rand_adj = noun
                .can_be
                .choose(&mut rng)
                .and_then(|id| adjs.get(id))
                .and_then(|v| v.choose(&mut rng))
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

    fn get_as_noun_complement(nouns: &'static StaticNouns) -> Box<dyn Fn(&Noun) -> WordGroup> {
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

    //let with_singular_adj = ;
    //let with_plural_adj = ;

    let mut constructions = [
        /*
         * ex:
         * une étoile d'été
         * les bruits d'aurore
         * un parfum de printemps
         * les odeurs d'automne
         */
        vec![
            (
                vec![
                    NounCatId::Astre,
                    NounCatId::Phenomene,
                    NounCatId::PhenomeneOlfactif,
                ],
                vec![
                    get_with_article(Article::Indefinite, Number::Singular),
                    get_with_article(Article::Indefinite, Number::Plural),
                    get_with_article(Article::Definite, Number::Singular),
                    get_with_article(Article::Definite, Number::Plural),
                ],
            ),
            (
                vec![NounCatId::Saison, NounCatId::MomentDuJour],
                vec![Box::new(get_apposition)],
            ),
        ],
        vec![
            (
                vec![NounCatId::Astre, NounCatId::PhenomeneLumineux],
                vec![
                    Box::new(extract_wordgroup),
                    get_with_adjective(&ADJ_CATS, Article::Definite, Number::Singular),
                    get_with_adjective(&ADJ_CATS, Article::Definite, Number::Plural),
                    get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Singular),
                    get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Plural),
                ],
            ),
            (
                vec![NounCatId::MomentDuJour],
                vec![Box::new(get_apposition)],
            ),
        ], /*
           vec![(NounCatId::Astre, with_singular_adj)],

           */
        vec![(
            vec![NounCatId::OrganeDePlante, NounCatId::Oiseau],
            vec![get_as_noun_complement(&NOUNS)],
        )],
        vec![(
            vec![
                NounCatId::PlanteAFleur,
                NounCatId::Plante,
                NounCatId::Oiseau,
            ],
            vec![
                get_with_adjective(&ADJ_CATS, Article::Definite, Number::Singular),
                get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Singular),
                get_with_adjective(&ADJ_CATS, Article::Definite, Number::Plural),
                get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Singular),
            ],
        )],
    ];
    constructions.shuffle(&mut rng);

    let mut haiku = [String::from(""), String::from(""), String::from("")];
    for nb in 0..3 {
        let mut is_running = true;
        while is_running {
            let random_result: Vec<Option<WordGroup>> = match constructions.get(nb) {
                Some(choice) => choice
                    .iter()
                    .map(|item| {
                        item.0
                            .choose(&mut rng)
                            .and_then(|choice| NOUN_CATS.get(&choice))
                            .and_then(|cat| cat.choose(&mut rng))
                            .and_then(|id| NOUNS.iter().find(|item| &item.id == id))
                            .and_then(|noun| {
                                item.1
                                    .choose(&mut rng)
                                    .and_then(|callback| Some(callback(noun)))
                            })
                    })
                    .collect(),
                None => vec![],
            };

            let result = random_result.iter().fold(
                WordGroup {
                    text: String::from(""),
                    foots: (0, 0),
                },
                |acc, wg_option| match wg_option {
                    Some(wg) => add_words(
                        acc,
                        WordGroup {
                            text: [" ", &wg.text].join(""),
                            foots: wg.foots,
                        },
                    ),
                    None => acc,
                },
            );

            match check_haiku_form([5, 7, 5], nb, &result) {
                Some(res) => {
                    haiku[nb] = res;
                    is_running = false;
                }
                None => (),
            }
            println!("{} ({} - {})", result.text, result.foots.0, result.foots.1);
        }
    }
    println!("Haïku:\n\t{}\n\t{}\n\t{}", haiku[0], haiku[1], haiku[2],);
}
