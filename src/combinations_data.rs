use crate::common_enums;
use common_enums::{Article, Number};

use crate::combinations;
use combinations::{
    get_as_noun_complement, get_with_adjective, get_with_intransitive_verb, get_with_rand_article,
    Construction,
};

// nouns
use crate::noun;
use crate::noun_enums;
use noun::{extract_wordgroup, get_apposition};
use noun_enums::NounCatId;
use crate::noun_data;
use noun_data::{NOUNS};

// verbs
use crate::verb_enums;
use verb_enums::VerbCatId;

// adjectives
use crate::adj_data;
use adj_data::ADJ_CATS;

pub type Constructions = [Construction; 5];

pub fn get_constructions() -> Constructions {
    [
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
                get_with_adjective(&ADJ_CATS, Article::Definite, Number::Singular),
                get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Singular),
                get_with_adjective(&ADJ_CATS, Article::Definite, Number::Plural),
                get_with_adjective(&ADJ_CATS, Article::Indefinite, Number::Singular),
            ],
        )],
    ]
}
