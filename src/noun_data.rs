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

use crate::verb;
use crate::verb_enums;
use verb::{get_verb, get_verb_cat};
use verb_enums::VerbCatId;

// EXPORTS
pub type StaticNouns = [Noun; 50];
lazy_static! {
    pub static ref NOUNS: StaticNouns = [
        Noun::new(NounId::Lune, "lune", Gender::Female, (1, 2),),
        Noun::new(NounId::Soleil, "soleil", Gender::Male, (2, 2),),
        Noun::new(NounId::Etoile, "étoile", Gender::Female, (2, 3),),
        Noun::new(NounId::Bruit, "bruit", Gender::Male, (1, 2)),
        Noun::new(NounId::Chant, "chant", Gender::Male, (1, 1)),
        Noun::new(NounId::Bruissement, "bruissement", Gender::Male, (2, 3),),
        Noun::new(NounId::Lumiere, "lumière", Gender::Female, (2, 3),),
        Noun::new(NounId::Rayon, "rayon", Gender::Male, (2, 2),),
        Noun::new(NounId::Odeur, "odeur", Gender::Female, (2, 2),),
        Noun::new(NounId::Parfum, "parfum", Gender::Male, (2, 2),),
        Noun::new(NounId::Arome, "arôme", Gender::Male, (2, 3),),
        Noun::new(NounId::Printemps, "printemps", Gender::Male, (2, 2),),
        Noun::new(NounId::Ete, "été", Gender::Male, (2, 2),),
        Noun::new(NounId::Automne, "automne", Gender::Male, (2, 2),),
        Noun::new(NounId::Hiver, "hiver", Gender::Male, (2, 2),),
        Noun::new(NounId::Prunier, "prunier", Gender::Male, (2, 3),),
        Noun::new(NounId::Cerisier, "cerisier", Gender::Male, (3, 4),),
        Noun::new(NounId::Oeillet, "oeillet", Gender::Male, (2, 2),),
        Noun::new(NounId::Glycine, "glycine", Gender::Female, (2, 2),),
        Noun::new(NounId::Pivoine, "pivoine", Gender::Female, (2, 3),),
        Noun::new(NounId::Feuille, "feuille", Gender::Male, (1, 2),),
        Noun::new(NounId::Branche, "branche", Gender::Female, (1, 2),),
        Noun::new(NounId::Aurore, "aurore", Gender::Female, (2, 2),),
        Noun::new(NounId::Crepuscule, "crépuscule", Gender::Male, (3, 4),),
        Noun::new(NounId::Midi, "midi", Gender::Male, (2, 2),),
        Noun::new(NounId::Minuit, "minuit", Gender::Male, (2, 2),),
        Noun::new(NounId::Mousse, "mousse", Gender::Female, (1, 2),),
        Noun::new(NounId::Liane, "liane", Gender::Female, (1, 2),),
        Noun::new(NounId::Lierre, "lierre", Gender::Male, (1, 2),),
        Noun::new(NounId::Chevrefeuille, "chèvrefeuille", Gender::Male, (3, 4),),
        Noun::new(NounId::Petale, "pétale", Gender::Male, (2, 3),),
        Noun::new(NounId::Etamine, "étamine", Gender::Female, (3, 4),),
        Noun::new(NounId::Alouette, "alouette", Gender::Female, (3, 4),),
        Noun::new(NounId::Mesange, "mésange", Gender::Female, (2, 3),),
        Noun::new(NounId::Grive, "grive", Gender::Female, (1, 2),),
        Noun::new(NounId::Canard, "canard", Gender::Male, (2, 2),),
        Noun::new(NounId::RougeGorge, "rougegorge", Gender::Male, (2, 3),),
        Noun::new(NounId::Fauvette, "fauvette", Gender::Female, (2, 3),),
        Noun::new(NounId::Hirondelle, "hirondelle", Gender::Female, (3, 4),),
        Noun::new(NounId::Merle, "merle", Gender::Male, (1, 2),),
        Noun::new(NounId::Pic, "pic", Gender::Male, (1, 1),),
        Noun::new(NounId::Cerf, "cerf", Gender::Male, (1, 1),),
        Noun::new(NounId::Biche, "biche", Gender::Female, (1, 2),),
        Noun::new(NounId::Faon, "faon", Gender::Male, (1, 1),),
        Noun::new(NounId::Ecureuil, "écureuil", Gender::Male, (3, 3),),
        Noun::new(NounId::Belette, "belette", Gender::Female, (2, 3),),
        Noun::new(NounId::Hermine, "hermine", Gender::Female, (2, 3),),
        Noun::new(NounId::Daim, "daim", Gender::Male, (1, 1),),
        Noun::new(NounId::Chevreuil, "chevreuil", Gender::Male, (2, 2),),
        Noun::new(NounId::Renard, "renard", Gender::Male, (2, 2),),
    ];
}
/**
 * Attributs et épithèthes sont séparé,
 * car "le cerf est grand" est peu poétique,
 * là ou "le grand cerf" l'est.
 */
#[derive(Clone)]
pub struct NounRelations {
    pub attributes: Vec<AdjCatId>, // EST - après verbe d'état (le cerf est grand)
    pub epithetes: Vec<AdjCatId>,  // PEUT ÊTRE - dans groupe nominal (le grand cerf)
    pub fonctions: Vec<VerbCatId>, // PEUT FAIRE - le cerf marche
    pub emissions: Vec<NounCatId>, // PEUT emettre - l'odeur du cerf
}

#[derive(Clone)]
pub struct NounCategory {
    pub id: NounCatId,
    pub inherit: Vec<NounCatId>,
    pub nouns: Vec<NounId>,
    pub rel: NounRelations,
}
impl NounCategory {
    pub fn has_intransitive_verb(&self) -> bool {
        self.rel.fonctions.len() > 0 && self.rel
            .fonctions
            .iter()
            .any(|verb_cat_id| match get_verb_cat(verb_cat_id) {
                Ok(verb_cat) => verb_cat
                    .iter()
                    .any(|verb_id| match get_verb(verb_id.clone()) {
                        Ok(verb) => true,
                        Err(_) => false,
                    }),
                Err(_) => false,
            })
    }
}

pub type NounCatHashMap = HashMap<NounCatId, NounCategory>;
lazy_static! {
    pub static ref NOUN_CATS: NounCatHashMap = [
        (
            NounCatId::Astre,
            NounCategory {
                id: NounCatId::Astre,
                inherit: vec![],
                nouns: vec![NounId::Lune, NounId::Soleil, NounId::Etoile],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Calme, AdjCatId::Chaleur],
                    epithetes: vec![
                        AdjCatId::RelAUneSaison,
                        AdjCatId::Coloration,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Grandeur,
                        AdjCatId::Calme,
                        AdjCatId::Chaleur,
                    ],
                    fonctions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::PhenomeneLumineux],
                },
            },
        ),
        (
            NounCatId::Phenomene,
            NounCategory {
                id: NounCatId::Phenomene,
                inherit: vec![],
                nouns: vec![NounId::Bruit, NounId::Lumiere, NounId::Odeur],
                rel: NounRelations {
                    epithetes: vec![],
                    attributes: vec![],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::PhenomeneLumineux,
            NounCategory {
                id: NounCatId::PhenomeneLumineux,
                inherit: vec![],
                nouns: vec![NounId::Lumiere, NounId::Rayon],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Coloration, AdjCatId::Chaleur],
                    epithetes: vec![AdjCatId::Coloration, AdjCatId::Chaleur],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::PhenomeneSonore,
            NounCategory {
                id: NounCatId::PhenomeneSonore,
                inherit: vec![],
                nouns: vec![NounId::Bruit, NounId::Chant],
                rel: NounRelations {
                    attributes: vec![],
                    epithetes: vec![],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::PhenomeneSonoreFloral,
            NounCategory {
                id: NounCatId::PhenomeneSonoreFloral,
                inherit: vec![],
                nouns: vec![NounId::Bruit, NounId::Bruissement],
                rel: NounRelations {
                    attributes: vec![],
                    epithetes: vec![],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::PhenomeneOlfactif,
            NounCategory {
                id: NounCatId::PhenomeneOlfactif,
                inherit: vec![],
                nouns: vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
                rel: NounRelations {
                    attributes: vec![],
                    epithetes: vec![],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::PhenomeneOlfactif,
            NounCategory {
                id: NounCatId::PhenomeneOlfactif,
                inherit: vec![],
                nouns: vec![NounId::Odeur, NounId::Parfum, NounId::Arome],
                rel: NounRelations {
                    attributes: vec![],
                    epithetes: vec![],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::Saison,
            NounCategory {
                id: NounCatId::Saison,
                inherit: vec![],
                nouns: vec![
                    NounId::Printemps,
                    NounId::Ete,
                    NounId::Automne,
                    NounId::Hiver,
                ],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Coloration],
                    epithetes: vec![AdjCatId::Coloration],
                    fonctions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::Phenomene],
                },
            },
        ),
        (
            NounCatId::SaisonChaude,
            NounCategory {
                id: NounCatId::SaisonChaude,
                inherit: vec![NounCatId::Saison],
                nouns: vec![NounId::Printemps, NounId::Ete, NounId::Automne,],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Chaleur],
                    epithetes: vec![AdjCatId::Chaleur],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::SaisonFroide,
            NounCategory {
                id: NounCatId::SaisonFroide,
                inherit: vec![NounCatId::Saison],
                nouns: vec![NounId::Printemps, NounId::Hiver, NounId::Automne,],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Froideur],
                    epithetes: vec![AdjCatId::Froideur],
                    fonctions: vec![],
                    emissions: vec![],
                },
            },
        ),
        (
            NounCatId::Plante,
            NounCategory {
                id: NounCatId::Plante,
                inherit: vec![],
                nouns: vec![
                    NounId::Mousse,
                    NounId::Liane,
                    NounId::Lierre,
                    NounId::Chevrefeuille,
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::Liberte,
                        AdjCatId::Coloration,
                        AdjCatId::ColorationRousse
                    ],
                    epithetes: vec![
                        AdjCatId::Liberte,
                        AdjCatId::Coloration,
                        AdjCatId::ColorationRousse
                    ],
                    fonctions: vec![],
                    emissions: vec![NounCatId::PhenomeneOlfactif],
                },
            },
        ),
        (
            NounCatId::PlanteAFleur,
            NounCategory {
                id: NounCatId::PlanteAFleur,
                inherit: vec![],
                nouns: vec![
                    NounId::Prunier,
                    NounId::Cerisier,
                    NounId::Oeillet,
                    NounId::Glycine,
                    NounId::Pivoine,
                    NounId::Chevrefeuille,
                ],
                rel: NounRelations {
                    attributes: vec![AdjCatId::EtatDeFloraison, AdjCatId::Liberte],
                    epithetes: vec![
                        AdjCatId::EtatDeFloraison,
                        AdjCatId::Liberte,
                        AdjCatId::Coloration
                    ],
                    fonctions: vec![],
                    emissions: vec![NounCatId::PhenomeneOlfactif],
                },
            },
        ),
        (
            NounCatId::OrganeDePlante,
            NounCategory {
                id: NounCatId::OrganeDePlante,
                inherit: vec![],
                nouns: vec![
                    NounId::Feuille,
                    NounId::Branche,
                    NounId::Petale,
                    NounId::Etamine,
                ],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Coloration],
                    epithetes: vec![AdjCatId::Coloration],
                    fonctions: vec![],
                    emissions: vec![
                        NounCatId::PhenomeneSonoreFloral,
                        NounCatId::PhenomeneOlfactif
                    ],
                },
            },
        ),
        (
            NounCatId::MomentDuJour,
            NounCategory {
                id: NounCatId::MomentDuJour,
                inherit: vec![],
                nouns: vec![
                    NounId::Aurore,
                    NounId::Crepuscule,
                    NounId::Midi,
                    NounId::Minuit
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::Coloration,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Chaleur,
                    ],
                    epithetes: vec![
                        AdjCatId::Coloration,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Chaleur,
                    ],
                    fonctions: vec![],
                    emissions: vec![
                        NounCatId::PhenomeneLumineux,
                        NounCatId::PhenomeneOlfactif,
                        NounCatId::Phenomene
                    ],
                },
            },
        ),
        (
            NounCatId::Oiseau,
            NounCategory {
                id: NounCatId::Oiseau,
                inherit: vec![],
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
                    attributes: vec![AdjCatId::Calme, AdjCatId::Agitation],
                    epithetes: vec![AdjCatId::Coloration, AdjCatId::Noblesse],
                    fonctions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::PhenomeneSonore],
                },
            },
        ),
        (
            NounCatId::Mammifere,
            NounCategory {
                id: NounCatId::Mammifere,
                inherit: vec![],
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
                    attributes: vec![AdjCatId::Calme, AdjCatId::Agitation],
                    epithetes: vec![
                        AdjCatId::ColorationRousse,
                        AdjCatId::Noblesse,
                        AdjCatId::Grandeur
                    ],
                    fonctions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![],
                },
            },
        )
    ]
    .iter()
    .cloned()
    .collect();
}
