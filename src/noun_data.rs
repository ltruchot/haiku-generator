// IMPORTS
use std::collections::HashMap;

use crate::common_enums;
use common_enums::Gender;

use crate::adj_enums;
use adj_enums::AdjCatId;

use crate::noun;
use crate::noun_enums;
use noun::Noun;
use noun_enums::{NounCatId, NounId};

use crate::verb_enums;
use verb_enums::{VerbCatId};

// EXPORTS
pub type StaticNouns = [Noun; 50];
lazy_static! {
    pub static ref NOUNS: StaticNouns = [
        Noun::new(
            NounId::Lune,
            "lune",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![
                AdjCatId::RelAUneSaison,
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Soleil,
            "soleil",
            Gender::Male,
            vec![NounCatId::PhenomeneLumineux],
            vec![
                AdjCatId::RelAUneSaison,
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Etoile,
            "étoile",
            Gender::Female,
            vec![NounCatId::PhenomeneLumineux],
            vec![AdjCatId::RelAUneSaison, AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(NounId::Bruit, "bruit", Gender::Male, vec![], vec![], vec![], (1, 2)),
        Noun::new(NounId::Chant, "chant", Gender::Male, vec![], vec![], vec![], (1, 1)),
        Noun::new(
            NounId::Bruissement,
            "bruissement",
            Gender::Male,
            vec![],
            vec![],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Lumiere,
            "lumière",
            Gender::Female,
            vec![],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Rayon,
            "rayon",
            Gender::Male,
            vec![],
            vec![AdjCatId::Coloration],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Odeur,
            "odeur",
            Gender::Female,
            vec![],
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
            vec![],
            (2, 2),
        ),
        Noun::new(NounId::Arome, "arôme", Gender::Male, vec![], vec![], vec![], (2, 3),),
        Noun::new(
            NounId::Printemps,
            "printemps",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Ete,
            "été",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Automne,
            "automne",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Hiver,
            "hiver",
            Gender::Male,
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Prunier,
            "prunier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EtatDeFloraison, AdjCatId::Liberte],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Cerisier,
            "cerisier",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EtatDeFloraison, AdjCatId::Liberte, AdjCatId::Grandeur],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Oeillet,
            "oeillet",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![
                AdjCatId::EtatDeFloraison,
                AdjCatId::Liberte,
                AdjCatId::Coloration
            ],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Glycine,
            "glycine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![
                AdjCatId::EtatDeFloraison,
                AdjCatId::Liberte,
                AdjCatId::Coloration
            ],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Pivoine,
            "pivoine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![
                AdjCatId::EtatDeFloraison,
                AdjCatId::Liberte,
                AdjCatId::Coloration
            ],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Feuille,
            "feuille",
            Gender::Male,
            vec![
                NounCatId::PhenomeneSonoreFloral,
                NounCatId::PhenomeneOlfactif
            ],
            vec![AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Branche,
            "branche",
            Gender::Male,
            vec![
                NounCatId::PhenomeneSonoreFloral,
                NounCatId::PhenomeneOlfactif
            ],
            vec![AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Aurore,
            "aurore",
            Gender::Female,
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Crepuscule,
            "crépuscule",
            Gender::Male,
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Midi,
            "midi",
            Gender::Male,
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Minuit,
            "minuit",
            Gender::Male,
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Mousse,
            "mousse",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![
                AdjCatId::Liberte,
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse
            ],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Liane,
            "liane",
            Gender::Female,
            vec![],
            vec![AdjCatId::Liberte, AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Lierre,
            "lierre",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Liberte, AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Chevrefeuille,
            "chèvrefeuille",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Petale,
            "pétale",
            Gender::Male,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Etamine,
            "étamine",
            Gender::Female,
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Alouette,
            "alouette",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Mesange,
            "mésange",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Grive,
            "grive",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Canard,
            "canard",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::RougeGorge,
            "rougegorge",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Fauvette,
            "fauvette",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Hirondelle,
            "hirondelle",
            Gender::Female,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (3, 4),
        ),
        Noun::new(
            NounId::Merle,
            "merle",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Pic,
            "pic",
            Gender::Male,
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Cerf,
            "cerf",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse, AdjCatId::Grandeur],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Biche,
            "biche",
            Gender::Female,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Faon,
            "faon",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Ecureuil,
            "écureuil",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (3, 3),
        ),
        Noun::new(
            NounId::Belette,
            "belette",
            Gender::Female,
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Hermine,
            "hermine",
            Gender::Female,
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Daim,
            "daim",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse, AdjCatId::Grandeur],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Chevreuil,
            "chevreuil",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse, AdjCatId::Grandeur],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Renard,
            "renard",
            Gender::Male,
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse, AdjCatId::Grandeur],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
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
            vec![
                NounId::Aurore,
                NounId::Crepuscule,
                NounId::Midi,
                NounId::Minuit
            ],
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
        ),
        (
            NounCatId::Mammifere,
            vec![
                NounId::Cerf,
                NounId::Ecureuil,
                NounId::Belette,
                NounId::Hermine,
                NounId::Faon,
                NounId::Biche,
                NounId::Daim,
                NounId::Chevreuil,
                NounId::Renard,
            ]
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
