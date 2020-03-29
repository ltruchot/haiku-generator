use crate::enums;
use enums::{AdjCatId, Article, Gender, NounCatId, NounId, Number};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Noun {
    pub id: NounId,
    pub lemme: String,
    pub gender: Gender,
    pub emit: Vec<NounCatId>,
    pub can_be: Vec<AdjCatId>,
}

impl Noun {
    pub fn new(
        id: NounId,
        lemme: &str,
        gender: Gender,
        emissions: Vec<NounCatId>,
        adjs: Vec<AdjCatId>,
    ) -> Noun {
        Noun {
            id: id,
            lemme: String::from(lemme),
            gender: gender,
            emit: emissions,
            can_be: adjs,
        }
    }
}

pub fn extract_lemme(noun: &Noun) -> String {
    noun.lemme.clone()
}

pub fn get_with_some_article(article: Article, number: Number, noun: &Noun) -> String {
    let article = match number {
        Number::Plural => match article {
            Article::Definite => "les",
            Article::Indefinite => "des",
        },
        Number::Singular => match noun.gender {
            Gender::Male => match article {
                Article::Definite => "le",
                Article::Indefinite => "un",
            },
            Gender::Female => match article {
                Article::Definite => "la",
                Article::Indefinite => "une",
            },
        },
    };
    let mut word = String::new();
    word.push_str(article);
    word.push(' ');
    word.push_str(&match number {
        Number::Plural => {
            let mut plural = String::from(&noun.lemme);
            plural.push('s');
            plural
        }
        Number::Singular => String::from(&noun.lemme),
    });
    word
}

pub fn get_apposition(noun: &Noun) -> String {
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

lazy_static! {
    pub static ref NOUNS: [Noun; 21] = [
        Noun::new(
            NounId::Lune,
            "lune",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison],
        ),
        Noun::new(
            NounId::Soleil,
            "soleil",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison],
        ),
        Noun::new(
            NounId::Etoile,
            "étoile",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison],
        ),
        Noun::new(NounId::Bruit, "bruit", Gender::Male, vec![], vec![]),
        Noun::new(NounId::Chant, "chant", Gender::Male, vec![], vec![]),
        Noun::new(
            NounId::Bruissement,
            "bruissement",
            Gender::Male,
            vec![],
            vec![]
        ),
        Noun::new(NounId::Lumiere, "lumière", Gender::Female, vec![], vec![]),
        Noun::new(NounId::Rayon, "rayon", Gender::Male, vec![], vec![]),
        Noun::new(NounId::Odeur, "odeur", Gender::Female, vec![], vec![]),
        Noun::new(NounId::Parfum, "parfum", Gender::Male, vec![], vec![]),
        Noun::new(
            NounId::Printemps,
            "printemps",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![],
        ),
        Noun::new(
            NounId::Ete,
            "été",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![],
        ),
        Noun::new(
            NounId::Automne,
            "automne",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![],
        ),
        Noun::new(
            NounId::Hiver,
            "hiver",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![],
        ),
        Noun::new(
            NounId::Prunier,
            "prunier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
        ),
        Noun::new(
            NounId::Cerisier,
            "cerisier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
        ),
        Noun::new(
            NounId::Oeillet,
            "oeillet",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
        ),
        Noun::new(
            NounId::Glycine,
            "glycine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
        ),
        Noun::new(
            NounId::Pivoine,
            "pivoine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
        ),
        Noun::new(
            NounId::Feuille,
            "feuille",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore, NounCatId::PhenomeneOlfactif],
            vec![],
        ),
        Noun::new(
            NounId::Branche,
            "branche",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore, NounCatId::PhenomeneOlfactif],
            vec![],
        ),
    ];
}

pub type NounCatHashMap = HashMap<NounCatId, Vec<NounId>>;
lazy_static! {
    pub static ref NOUN_CATS: NounCatHashMap = [
        (
            NounCatId::Astre,
            vec![NounId::Lune, NounId::Soleil, NounId::Etoile],
        ),
        (
            NounCatId::Phenomene,
            vec![NounId::Bruit, NounId::Lumiere, NounId::Odeur],
        ),
        (
            NounCatId::PhenomeneLumineux,
            vec![NounId::Lumiere, NounId::Rayon],
        ),
        (
            NounCatId::PhenomeneSonore,
            vec![NounId::Bruit, NounId::Chant, NounId::Bruissement],
        ),
        (
            NounCatId::PhenomeneOlfactif,
            vec![NounId::Odeur, NounId::Parfum],
        ),
        (
            NounCatId::Saison,
            vec![
                NounId::Printemps,
                NounId::Ete,
                NounId::Automne,
                NounId::Hiver,
            ],
        ),
        (
            NounCatId::PlanteAFleur,
            vec![
                NounId::Prunier,
                NounId::Cerisier,
                NounId::Oeillet,
                NounId::Glycine,
                NounId::Pivoine,
            ],
        ),
        (
            NounCatId::OrganeDePlante,
            vec![NounId::Feuille, NounId::Branche,],
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
