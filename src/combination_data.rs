// IMPORTS

// commons
use crate::common_enums;
use common_enums::{Number, Article};

use crate::combination;
use combination::{
    get_as_noun_complement, get_with_adjective, get_with_linking_verb, get_with_rand_article,
    get_with_verb, Construction,
};

// nouns
use crate::noun;
use crate::noun_data;
use crate::noun_enums;
use noun::{extract_wordgroup, get_apposition};
use noun_data::NOUNS;
use noun_enums::NounCatId;

// verbs
use crate::verb_enums;
use verb_enums::VerbCatId;

// EXPORTS
pub type Constructions = [Construction; 6];

pub fn get_constructions() -> Constructions {
    [
        vec![(
            vec![NounCatId::Astre, NounCatId::Mammifere],
            vec![get_with_verb(vec![VerbCatId::EtatDEveil], Number::Singular)],
        )],
        vec![(
            vec![NounCatId::Astre, NounCatId::Mammifere],
            vec![get_with_linking_verb(
                vec![VerbCatId::Etat],
                Some(Number::Singular),
                Some(Article::Definite),
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
                vec![NounCatId::Astre, NounCatId::PhenomeneLumineux, NounCatId::Saison],
                vec![
                    Box::new(extract_wordgroup),
                    get_with_adjective(Article::Definite, Number::Singular),
                    get_with_adjective(Article::Definite, Number::Plural),
                    get_with_adjective(Article::Indefinite, Number::Singular),
                    get_with_adjective(Article::Indefinite, Number::Plural),
                ],
            ),
            (
                vec![NounCatId::MomentDuJour],
                vec![Box::new(get_apposition)],
            ),
        ],
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
                get_with_adjective(Article::Definite, Number::Singular),
                get_with_adjective(Article::Indefinite, Number::Singular),
                get_with_adjective(Article::Definite, Number::Plural),
                get_with_adjective(Article::Indefinite, Number::Singular),
            ],
        )],
    ]
}
