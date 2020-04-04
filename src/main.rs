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
    get_as_noun_complement, 
    get_with_adjective,
    get_with_rand_article,
    get_with_intransitive_verb,
    Combination,
};

// nouns
mod noun;
use noun::{extract_wordgroup, get_apposition, pick_rand_noun};
mod noun_enums;
use noun_enums::{NounCatId, NounId};
mod noun_data;
use noun_data::{NOUNS, NOUN_CATS};


// adjectives
mod adj;
mod adj_data;
mod adj_enums;
use adj_data::ADJ_CATS;

// verbs
mod verb;
mod verb_data;
mod verb_enums;
use verb_enums::VerbCatId;

fn main() {
    let mut rng = thread_rng();
    let mut noun_black_list: Vec<NounId> = vec![];

    let mut constructions: [Vec<(Vec<NounCatId>, Vec<Combination>)>; 5] = [
        vec![(
            vec![NounCatId::Astre, NounCatId::Mammifere],
            vec![get_with_intransitive_verb(
                vec![VerbCatId::EtatDEveil],
                Number::Singular,
            )],
        )],
        vec![
            (
                vec![
                    NounCatId::Astre,
                    NounCatId::Phenomene,
                    NounCatId::PhenomeneOlfactif,
                ],
                vec![get_with_rand_article()],
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
                NounCatId::Mammifere,
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
        let mut current_noun_id: Option<NounId> = None;
        while is_running {
            let random_result: Vec<Option<WordGroup>> = match constructions.get(nb) {
                Some(choice) => choice
                    .iter()
                    .map(|item| {
                        pick_rand_noun(&item.0, &mut rng, &noun_black_list)
                            .and_then(|noun| {
                                current_noun_id = Some(noun.id);
                                item.1
                                    .choose(&mut rng)
                                    .and_then(|callback| Some(callback(&noun)))
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
                    Some(wg) => add_words(&acc, &wg, true),
                    None => acc,
                },
            );

            match check_haiku_form([5, 7, 5], nb, &result) {
                Some(res) => {
                    haiku[nb] = res;
                    is_running = false;
                    match current_noun_id {
                        Some(id) => noun_black_list.push(id),
                        None => ()
                    }   
                }
                None => (),
            }
            println!("{} ({} - {})", result.text, result.foots.0, result.foots.1);
        }
    }
    println!("{:?}", noun_black_list);
    println!("Haïku:\n\t{}\n\t{}\n\t{}", haiku[0], haiku[1], haiku[2],);
}
