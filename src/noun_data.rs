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
use verb_enums::VerbCatId;

// EXPORTS
pub type StaticNouns = [Noun; 50];
lazy_static! {
    pub static ref NOUNS: StaticNouns = [
        Noun::new(
            NounId::Lune,
            "lune",
            Gender::Female,
            vec![AdjCatId::Calme],
            vec![NounCatId::PhenomeneLumineux],
            vec![
                AdjCatId::RelAUneSaison,
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse,
                AdjCatId::Grandeur,
                AdjCatId::Calme,
                AdjCatId::Chaleur,
            ],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Soleil,
            "soleil",
            Gender::Male,
            vec![AdjCatId::Calme],
            vec![NounCatId::PhenomeneLumineux],
            vec![
                AdjCatId::RelAUneSaison,
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse,
                AdjCatId::Grandeur,
                AdjCatId::Calme,
                AdjCatId::Chaleur,
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Etoile,
            "étoile",
            Gender::Female,
            vec![AdjCatId::Calme],
            vec![NounCatId::PhenomeneLumineux],
            vec![
                AdjCatId::RelAUneSaison,
                AdjCatId::Coloration,
                AdjCatId::Calme,
                AdjCatId::Chaleur
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Bruit,
            "bruit",
            Gender::Male,
            vec![],
            vec![],
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
            vec![],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Lumiere,
            "lumière",
            Gender::Female,
            vec![],
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Rayon,
            "rayon",
            Gender::Male,
            vec![],
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
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
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Arome,
            "arôme",
            Gender::Male,
            vec![],
            vec![],
            vec![],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Printemps,
            "printemps",
            Gender::Male,
            vec![],
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Ete,
            "été",
            Gender::Male,
            vec![],
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Automne,
            "automne",
            Gender::Male,
            vec![],
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Hiver,
            "hiver",
            Gender::Male,
            vec![],
            vec![NounCatId::Phenomene],
            vec![AdjCatId::Coloration],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Prunier,
            "prunier",
            Gender::Male,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::EtatDeFloraison, AdjCatId::Liberte],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Cerisier,
            "cerisier",
            Gender::Male,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![
                AdjCatId::EtatDeFloraison,
                AdjCatId::Liberte,
                AdjCatId::Grandeur
            ],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Oeillet,
            "oeillet",
            Gender::Male,
            vec![],
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
            vec![],
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
            vec![],
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
            vec![],
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
            vec![],
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
            vec![],
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse, AdjCatId::Chaleur,],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Crepuscule,
            "crépuscule",
            Gender::Male,
            vec![AdjCatId::Calme],
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse, AdjCatId::Chaleur,],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Midi,
            "midi",
            Gender::Male,
            vec![],
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Minuit,
            "minuit",
            Gender::Male,
            vec![],
            vec![
                NounCatId::PhenomeneLumineux,
                NounCatId::PhenomeneOlfactif,
                NounCatId::Phenomene
            ],
            vec![AdjCatId::Coloration, AdjCatId::Chaleur,],
            vec![],
            (2, 2),
        ),
        Noun::new(
            NounId::Mousse,
            "mousse",
            Gender::Female,
            vec![],
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
            vec![],
            vec![AdjCatId::Liberte, AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Lierre,
            "lierre",
            Gender::Male,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Liberte, AdjCatId::Coloration],
            vec![],
            (1, 2),
        ),
        Noun::new(
            NounId::Chevrefeuille,
            "chèvrefeuille",
            Gender::Male,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Petale,
            "pétale",
            Gender::Male,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Etamine,
            "étamine",
            Gender::Female,
            vec![],
            vec![NounCatId::PhenomeneOlfactif],
            vec![AdjCatId::Coloration],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Alouette,
            "alouette",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![],
            (3, 4),
        ),
        Noun::new(
            NounId::Mesange,
            "mésange",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![],
            (2, 3),
        ),
        Noun::new(
            NounId::Grive,
            "grive",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Canard,
            "canard",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::RougeGorge,
            "rougegorge",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Fauvette,
            "fauvette",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![
                AdjCatId::Coloration,
                AdjCatId::ColorationRousse,
                AdjCatId::Noblesse
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Hirondelle,
            "hirondelle",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (3, 4),
        ),
        Noun::new(
            NounId::Merle,
            "merle",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Pic,
            "pic",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![NounCatId::PhenomeneSonore],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Cerf,
            "cerf",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![
                AdjCatId::ColorationRousse,
                AdjCatId::Noblesse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Biche,
            "biche",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (1, 2),
        ),
        Noun::new(
            NounId::Faon,
            "faon",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![AdjCatId::ColorationRousse],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Ecureuil,
            "écureuil",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![AdjCatId::ColorationRousse, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (3, 3),
        ),
        Noun::new(
            NounId::Belette,
            "belette",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::ColorationRousse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Hermine,
            "hermine",
            Gender::Female,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![AdjCatId::Coloration, AdjCatId::Noblesse],
            vec![VerbCatId::EtatDEveil],
            (2, 3),
        ),
        Noun::new(
            NounId::Daim,
            "daim",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![
                AdjCatId::ColorationRousse,
                AdjCatId::Noblesse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (1, 1),
        ),
        Noun::new(
            NounId::Chevreuil,
            "chevreuil",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![
                AdjCatId::ColorationRousse,
                AdjCatId::Noblesse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
        Noun::new(
            NounId::Renard,
            "renard",
            Gender::Male,
            vec![AdjCatId::Calme, AdjCatId::Agitation],
            vec![],
            vec![
                AdjCatId::ColorationRousse,
                AdjCatId::Noblesse,
                AdjCatId::Grandeur
            ],
            vec![VerbCatId::EtatDEveil],
            (2, 2),
        ),
    ];
}
/**
 * Attributs et épithèthes sont séparé, 
 * car "le cerf est grand" est peu poétique, 
 * là ou "le grand cerf" l'est.
 */
#[derive(Clone)]
pub struct NounRelations {
    pub epithetes: Vec<AdjCatId>, // PEUT ÊTRE - dans groupe nominal (le grand cerf) 
    pub attributes: Vec<AdjCatId>, // EST - après verbe d'état (le cerf est grand)
    pub fonctions: Vec<VerbCatId>, // PEUT FAIRE - le cerf marche
    pub emissions: Vec<NounCatId>, // PEUT emettre - l'odeur du cerf
}

#[derive(Clone)]
pub struct NounCategory {
    pub id: NounCatId,
    pub nouns: Vec<NounId>,
    pub rel: NounRelations,
}

pub type NounCatHashMap = HashMap<NounCatId, NounCategory>;
lazy_static! {
    pub static ref NOUN_CATS: NounCatHashMap = [
        (NounCatId::Astre, 
        NounCategory {
            id: NounCatId::Astre,
            nouns: vec![NounId::Lune, NounId::Soleil, NounId::Etoile],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),
    (NounCatId::Phenomene,
        NounCategory {
            id: NounCatId::Phenomene,
            nouns: vec![NounId::Bruit, NounId::Lumiere, NounId::Odeur],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PhenomeneLumineux,
        NounCategory {
            id: NounCatId::PhenomeneLumineux,
            nouns: vec![NounId::Lumiere, NounId::Rayon],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PhenomeneSonore,
        NounCategory {
            id: NounCatId::PhenomeneSonore,
            nouns: vec![NounId::Bruit, NounId::Chant],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PhenomeneSonoreFloral,
        NounCategory {
            id: NounCatId::PhenomeneSonoreFloral,
            nouns: vec![NounId::Bruit, NounId::Bruissement],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PhenomeneOlfactif,
        NounCategory {
            id: NounCatId::PhenomeneOlfactif,
            nouns: vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PhenomeneOlfactif,
        NounCategory {
            id: NounCatId::PhenomeneOlfactif,
            nouns: vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::Saison,
        NounCategory {
            id: NounCatId::Saison,
            nouns: vec![
                NounId::Printemps,
                NounId::Ete,
                NounId::Automne,
                NounId::Hiver,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::PlanteAFleur,
        NounCategory {
            id: NounCatId::PlanteAFleur,
            nouns: vec![
                NounId::Prunier,
                NounId::Cerisier,
                NounId::Oeillet,
                NounId::Glycine,
                NounId::Pivoine,
                NounId::Chevrefeuille,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::OrganeDePlante,
        NounCategory {
            id: NounCatId::OrganeDePlante,
            nouns: vec![
                NounId::Feuille,
                NounId::Branche,
                NounId::Petale,
                NounId::Etamine,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::MomentDuJour,
        NounCategory {
            id: NounCatId::MomentDuJour,
            nouns: vec![
                NounId::Aurore,
                NounId::Crepuscule,
                NounId::Midi,
                NounId::Minuit
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::Plante,
        NounCategory {
            id: NounCatId::Plante,
            nouns: vec![
                NounId::Mousse,
                NounId::Liane,
                NounId::Lierre,
                NounId::Chevrefeuille,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::Oiseau,
        NounCategory {
            id: NounCatId::Oiseau,
            nouns: vec![
                NounId::Alouette,
                NounId::Mesange,
                NounId::Grive,
                NounId::Canard,
                NounId::RougeGorge,
                NounId::Fauvette,
                NounId::Hirondelle,
                NounId::Merle,
                NounId::Pic,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    ),(NounCatId::Mammifere,
        NounCategory {
            id: NounCatId::Mammifere,
            nouns: vec![
                NounId::Cerf,
                NounId::Ecureuil,
                NounId::Belette,
                NounId::Hermine,
                NounId::Faon,
                NounId::Biche,
                NounId::Daim,
                NounId::Chevreuil,
                NounId::Renard,
            ],
            rel: NounRelations {
                epithetes: vec![],
                attributes: vec![],
                fonctions: vec![],
                emissions:vec![],
            },
        },
    )
    ]
    .iter()
    .cloned()
    .collect();
}
