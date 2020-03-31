// IMPORTS
use std::collections::HashMap;

use crate::common_enums;
use common_enums::{Gender};

use crate::adj_enums;
use adj_enums::{AdjCatId};

use crate::noun;
use noun::{Noun};
use crate::noun_enums;
use noun_enums::{NounCatId, NounId};

// EXPORTS
pub type StaticNouns = [Noun; 41];
lazy_static! {
    pub static ref NOUNS: StaticNouns = [
        Noun::new(
            NounId::Lune,
            "lune",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison, AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Soleil,
            "soleil",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison, AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Etoile,
            "étoile",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison, AdjCatId::Couleur],
            (2, 3),
        ),
        Noun::new(
            NounId::Bruit,
            "bruit",
            Gender::Male,
            vec![],
            vec![],
            (1, 2)
        ),
        Noun::new(
            NounId::Chant,
            "chant",
            Gender::Male,
            vec![],
            vec![],
            (1, 1)
        ),
        Noun::new(
            NounId::Bruissement,
            "bruissement",
            Gender::Male,
            vec![],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Lumiere,
            "lumière",
            Gender::Female,
            vec![],
            vec![AdjCatId::Couleur],
            (2, 3),
        ),
        Noun::new(
            NounId::Rayon,
            "rayon",
            Gender::Male,
            vec![],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Odeur,
            "odeur",
            Gender::Female,
            vec![],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Parfum,
            "parfum",
            Gender::Male,
            vec![],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Arome,
            "arôme",
            Gender::Male,
            vec![],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Printemps,
            "printemps",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Ete,
            "été",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Automne,
            "automne",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Hiver,
            "hiver",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Prunier,
            "prunier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
            (2, 3),
        ),
        Noun::new(
            NounId::Cerisier,
            "cerisier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage],
            (3, 4),
        ),
        Noun::new(
            NounId::Oeillet,
            "oeillet",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage, AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Glycine,
            "glycine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage, AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Pivoine,
            "pivoine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EnFleur, AdjCatId::Sauvage, AdjCatId::Couleur],
            (2, 3),
        ),
        Noun::new(
            NounId::Feuille,
            "feuille",
            Gender::Male,
            vec![NounCatId::PhenomeneSonoreFloral, NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Branche,
            "branche",
            Gender::Male,
            vec![NounCatId::PhenomeneSonoreFloral, NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Aurore,
            "aurore",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux, NounCatId::PhenomeneOlfactif, NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Crepuscule,
            "crépuscule",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux, NounCatId::PhenomeneOlfactif, NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (3, 4),
        ),
        Noun::new(
            NounId::Midi,
            "midi",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux, NounCatId::PhenomeneOlfactif, NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Minuit,
            "minuit",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux, NounCatId::PhenomeneOlfactif, NounCatId::Phenomene],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::Mousse,
            "mousse",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Sauvage, AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Liane,
            "liane",
            Gender::Female,
            vec![],
            vec![AdjCatId::Sauvage, AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Lierre,
            "lierre",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Sauvage, AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Chevrefeuille,
            "chèvrefeuille",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Couleur],
            (3, 4),
        ),
        Noun::new(
            NounId::Petale,
            "pétale",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Couleur],
            (2, 3),
        ),
        Noun::new(
            NounId::Etamine,
            "étamine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Couleur],
            (3, 4),
        ),
        Noun::new(
            NounId::Alouette,
            "alouette",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (3, 4),
        ),  
        Noun::new(
            NounId::Mesange,
            "mésange",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (2, 3),
        ),
        Noun::new(
            NounId::Grive,
            "grive",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (1, 2),
        ),  
        Noun::new(
            NounId::Canard,
            "canard",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (2, 2),
        ),
        Noun::new(
            NounId::RougeGorge,
            "rougegorge",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (2, 3),
        ),  
        Noun::new(
            NounId::Fauvette,
            "fauvette",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (2, 3),
        ),  
        Noun::new(
            NounId::Hirondelle,
            "hirondelle",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (3, 4),
        ),  
        Noun::new(
            NounId::Merle,
            "merle",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (1, 2),
        ),
        Noun::new(
            NounId::Pic,
            "pic",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Couleur],
            (1, 1),
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
            vec![NounId::Bruit, NounId::Chant],
        ),
        (
            NounCatId::PhenomeneSonoreFloral,
            vec![NounId::Bruit, NounId::Bruissement],
        ),
        (
            NounCatId::PhenomeneOlfactif,
            vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
        ),
        (
            NounCatId::PhenomeneOlfactif,
            vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
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
                NounId::Chevrefeuille,
            ],
        ),
        (
            NounCatId::OrganeDePlante,
            vec![
                NounId::Feuille, 
                NounId::Branche,
                NounId::Petale,
                NounId::Etamine,
            ],
        ),
        (
            NounCatId::MomentDuJour,
            vec![NounId::Aurore, NounId::Crepuscule, NounId::Midi, NounId::Minuit],
        ),
        (
            NounCatId::Plante,
            vec![
                NounId::Mousse,
                NounId::Liane,
                NounId::Lierre,
                NounId::Chevrefeuille,
            ]
        ),
        (
            NounCatId::Oiseau,
            vec![
                NounId::Alouette,
                NounId::Mesange,
                NounId::Grive,
                NounId::Canard,
                NounId::RougeGorge,
                NounId::Fauvette,
                NounId::Hirondelle,
                NounId::Merle,
                NounId::Pic,
            ]
        )
    ]
    .iter()
    .cloned()
    .collect();
}