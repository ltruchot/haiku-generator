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
mod combinations;
use combinations::{
    Combination, 
    get_with_article, 
    get_with_rand_article,
    get_with_adjective, 
    get_as_noun_complement
};

// nouns
mod noun;
use noun::{extract_wordgroup, get_apposition};
mod noun_enums;
use noun_enums::{NounCatId};
mod noun_data;
use noun_data::{NOUNS, NOUN_CATS};

// adjectives
mod adj;
mod adj_enums;
mod adj_data;
use adj_data::{ADJ_CATS};

// verbs
mod verb;
mod verb_enums;
mod verb_data;


fn main() {
    let mut rng = thread_rng();

    let mut constructions: [Vec<(Vec<NounCatId>, Vec<Combination>)>; 4] = [
        vec![
            (
                vec![
                    NounCatId::Astre,
                    NounCatId::Phenomene,
                    NounCatId::PhenomeneOlfactif,
                ],
                vec![
                    get_with_rand_article()
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
                NounCatId::Mammifere
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
