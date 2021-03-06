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
pub type StaticNouns = [Noun; 71];
lazy_static! {
    pub static ref NOUNS: StaticNouns = [
        Noun::new(NounId::Lune, "lune", Gender::Female, (1, 1),),
        Noun::new(NounId::Soleil, "soleil", Gender::Male, (2, 2),),
        Noun::new(NounId::Etoile, "étoile", Gender::Female, (2, 2),),
        Noun::new(NounId::Bruit, "bruit", Gender::Male, (1, 2)),
        Noun::new(NounId::Chant, "chant", Gender::Male, (1, 1)),
        Noun::new(NounId::Bruissement, "bruissement", Gender::Male, (3, 3),),
        Noun::new(NounId::Lumiere, "lumière", Gender::Female, (2, 3),),
        Noun::new(NounId::Rayon, "rayon", Gender::Male, (2, 2),),
        Noun::new(NounId::Odeur, "odeur", Gender::Female, (2, 2),),
        Noun::new(NounId::Parfum, "parfum", Gender::Male, (2, 2),),
        Noun::new(NounId::Arome, "arôme", Gender::Male, (2, 2),),
        Noun::new(NounId::Printemps, "printemps", Gender::Male, (2, 2),),
        Noun::new(NounId::Ete, "été", Gender::Male, (2, 2),),
        Noun::new(NounId::Automne, "automne", Gender::Male, (2, 2),),
        Noun::new(NounId::Hiver, "hiver", Gender::Male, (2, 2),),
        Noun::new(NounId::Prunier, "prunier", Gender::Male, (2, 3),),
        Noun::new(NounId::Cerisier, "cerisier", Gender::Male, (3, 4),),
        Noun::new(NounId::Oeillet, "oeillet", Gender::Male, (2, 2),),
        Noun::new(NounId::Glycine, "glycine", Gender::Female, (2, 2),),
        Noun::new(NounId::Pivoine, "pivoine", Gender::Female, (2, 2),),
        Noun::new(NounId::Feuille, "feuille", Gender::Female, (1, 1),),
        Noun::new(NounId::Branche, "branche", Gender::Female, (1, 1),),
        Noun::new(NounId::Aurore, "aurore", Gender::Female, (2, 2),),
        Noun::new(NounId::Crepuscule, "crépuscule", Gender::Male, (3, 3),),
        Noun::new(NounId::Midi, "midi", Gender::Male, (2, 2),),
        Noun::new(NounId::Minuit, "minuit", Gender::Male, (2, 2),),
        Noun::new(NounId::Mousse, "mousse", Gender::Female, (1, 1),),
        Noun::new(NounId::Liane, "liane", Gender::Female, (1, 1),),
        Noun::new(NounId::Lierre, "lierre", Gender::Male, (1, 1),),
        Noun::new(NounId::Chevrefeuille, "chèvrefeuille", Gender::Male, (3, 3),),
        Noun::new(NounId::Petale, "pétale", Gender::Male, (2, 2),),
        Noun::new(NounId::Etamine, "étamine", Gender::Female, (3, 3),),
        Noun::new(NounId::Alouette, "alouette", Gender::Female, (3, 3),),
        Noun::new(NounId::Mesange, "mésange", Gender::Female, (2, 2),),
        Noun::new(NounId::Grive, "grive", Gender::Female, (1, 1),),
        Noun::new(NounId::Canard, "canard", Gender::Male, (2, 2),),
        Noun::new(NounId::RougeGorge, "rougegorge", Gender::Male, (3, 3),),
        Noun::new(NounId::Fauvette, "fauvette", Gender::Female, (2, 2),),
        Noun::new(NounId::Hirondelle, "hirondelle", Gender::Female, (3, 3),),
        Noun::new(NounId::Merle, "merle", Gender::Male, (1, 1),),
        Noun::new(NounId::Pie, "pie", Gender::Female, (1, 1),),
        Noun::new(NounId::Cerf, "cerf", Gender::Male, (1, 1),),
        Noun::new(NounId::Biche, "biche", Gender::Female, (1, 1),),
        Noun::new(NounId::Faon, "faon", Gender::Male, (1, 1),),
        Noun::new(NounId::Ecureuil, "écureuil", Gender::Male, (3, 3),),
        Noun::new(NounId::Belette, "belette", Gender::Female, (2, 2),),
        Noun::new(NounId::Hermine, "hermine", Gender::Female, (2, 2),),
        Noun::new(NounId::Daim, "daim", Gender::Male, (1, 1),),
        Noun::new(NounId::Chevreuil, "chevreuil", Gender::Male, (2, 2),),
        Noun::new(NounId::Renard, "renard", Gender::Male, (2, 2),),
        Noun::new(NounId::Plume, "plume", Gender::Female, (1,1)),
        // Noun::new(NounId::Duvet, "duvet", Gender::Male, (2, 2)),
        Noun::new(NounId::Reflet, "reflet", Gender::Male, (2,2)),
        Noun::new(NounId::Eclat, "éclat", Gender::Male, (2, 2)),
        Noun::new(NounId::Eclat, "raie", Gender::Female, (1, 1)),
        Noun::new(NounId::Moment, "moment", Gender::Male, (2, 2)),
        Noun::new(NounId::Heure, "heure", Gender::Female, (1, 1)),
        Noun::new(NounId::Minute, "minute", Gender::Female, (2, 2)),
        Noun::new(NounId::Calme, "calme", Gender::Male, (1, 1)),
        Noun::new(NounId::Paix, "paix", Gender::Female, (1, 1)),
        Noun::new(NounId::Serenite, "sérénité", Gender::Female, (4, 4)),
        Noun::new(NounId::Pluie, "pluie", Gender::Female, (1, 1)),
        Noun::new(NounId::Bruine, "bruine", Gender::Female, (2, 2)),
        Noun::new(NounId::Rosee, "rosée", Gender::Female, (2, 2)),
        Noun::new(NounId::Orage, "orage", Gender::Male, (2, 2)),
        Noun::new(NounId::Averse, "averse", Gender::Female, (2, 2)),
        Noun::new(NounId::Embellie, "embellie", Gender::Female, (3, 3)),
        Noun::new(NounId::Brume, "brume", Gender::Female, (1, 1)),
        Noun::new(NounId::Saveur, "saveur", Gender::Female, (2, 2)),
        Noun::new(NounId::Esprit, "esprit", Gender::Male, (2, 2)),
        Noun::new(NounId::Ame, "âme", Gender::Female, (1, 1)),
        Noun::new(NounId::Coeur, "cœur", Gender::Male, (1, 1)),
    ];
}
/**
 * Attributs et épithèthes sont séparés,
 * car "le cerf est grand" est peu poétique,
 * là ou "le grand cerf" l'est.
 */
#[derive(Clone)]
pub struct NounRelations {
    pub attributes: Vec<AdjCatId>, // EST - après verbe d'état (le cerf est grand)
    pub epithets: Vec<AdjCatId>,   // PEUT ÊTRE - dans groupe nominal (le grand cerf)
    pub functions: Vec<VerbCatId>, // PEUT FAIRE - le cerf marche
    pub emissions: Vec<NounCatId>, // PEUT emettre - l'odeur du cerf
    pub affiliations: Vec<NounCatId>, // APPARTIENT À - après verbe d'état (le cerf du bois)
}

#[derive(Clone)]
pub struct NounCategory {
    pub id: NounCatId,
    pub nouns: Vec<NounId>,
    pub rel: NounRelations,
}
impl NounCategory {
    pub fn has_intransitive_verb(&self) -> bool {
        self.rel.functions.len() > 0
            && self
                .rel
                .functions
                .iter()
                .any(|verb_cat_id| match get_verb_cat(verb_cat_id) {
                    Ok(verb_cat) => {
                        verb_cat
                            .iter()
                            .any(|verb_id| match get_verb(verb_id.clone()) {
                                Ok(_) => true,
                                Err(_) => false,
                            })
                    }
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
                nouns: vec![NounId::Lune, NounId::Soleil, NounId::Etoile],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Calme, AdjCatId::Chaleur],
                    epithets: vec![
                        AdjCatId::RelAUneSaison,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Grandeur,
                        AdjCatId::Calme,
                        AdjCatId::Chaleur,
                        AdjCatId::Anonymat,
                    ],
                    functions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::PhenomeneLumineux, NounCatId::EffetLumineux],
                    affiliations: vec![
                        NounCatId::Saison,
                        NounCatId::MomentDuJour,
                        NounCatId::MiJour
                    ]
                },
            },
        ),
        (
            NounCatId::Phenomene,
            NounCategory {
                id: NounCatId::Phenomene,
                nouns: vec![NounId::Bruit, NounId::Lumiere, NounId::Odeur, NounId::Saveur],
                rel: NounRelations {
                    epithets: vec![],
                    attributes: vec![],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![
                        NounCatId::Saison,
                        NounCatId::MomentDuJour,
                        NounCatId::MiJour
                    ]
                },
            },
        ),
        (
            NounCatId::PhenomeneLumineux,
            NounCategory {
                id: NounCatId::PhenomeneLumineux,
                nouns: vec![NounId::Lumiere, NounId::Rayon],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Chaleur
                    ],
                    epithets: vec![
                        AdjCatId::Chaleur,
                        AdjCatId::ColorationRousse,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                    ],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![
                        NounCatId::Saison,
                        NounCatId::MomentDuJour,
                        NounCatId::MiJour
                    ]
                },
            },
        ),
        (
            NounCatId::PhenomeneSonore,
            NounCategory {
                id: NounCatId::PhenomeneSonore,
                nouns: vec![NounId::Bruit, NounId::Chant],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::Saison, NounCatId::MomentDuJour]
                },
            },
        ),
        (
            NounCatId::PhenomeneSonoreFloral,
            NounCategory {
                id: NounCatId::PhenomeneSonoreFloral,
                nouns: vec![NounId::Bruit, NounId::Bruissement],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::OrganeDePlante],
                },
            },
        ),
        (
            NounCatId::PhenomeneOlfactif,
            NounCategory {
                id: NounCatId::PhenomeneOlfactif,
                nouns: vec![NounId::Odeur, NounId::Parfum, NounId::Arome, NounId::Saveur],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![
                        AdjCatId::Chaleur,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::Delicatesse,
                        AdjCatId::Saveur,
                    ],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::MomentDuJour, NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::PhenomeneTactile,
            NounCategory {
                id: NounCatId::PhenomeneTactile,
                nouns: vec![NounId::Rosee],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::Chaleur,
                        AdjCatId::Froideur,
                    ],
                    epithets: vec![
                        AdjCatId::Chaleur,
                        AdjCatId::Froideur,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::Delicatesse,
                        AdjCatId::Saveur,
                    ],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::MomentDuJour, NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::Saison,
            NounCategory {
                id: NounCatId::Saison,
                nouns: vec![
                    NounId::Printemps,
                    NounId::Ete,
                    NounId::Automne,
                    NounId::Hiver,
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::ColorationBrillante,
                    ],
                    epithets: vec![
                        AdjCatId::ColorationRousse,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                    ],
                    functions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::Phenomene],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::SaisonChaude,
            NounCategory {
                id: NounCatId::SaisonChaude,
                nouns: vec![NounId::Printemps, NounId::Ete, NounId::Automne,],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Chaleur],
                    epithets: vec![AdjCatId::Chaleur,
                    AdjCatId::ColorationDiapree,
                    AdjCatId::ColorationBrillante,
                    AdjCatId::ColorationBleue,
                    AdjCatId::ColorationRousse,
                    ],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::SaisonFroide,
            NounCategory {
                id: NounCatId::SaisonFroide,
                nouns: vec![NounId::Printemps, NounId::Hiver, NounId::Automne,],
                rel: NounRelations {
                    attributes: vec![AdjCatId::Froideur],
                    epithets: vec![AdjCatId::Froideur,                        
                    AdjCatId::ColorationDiapree,
                    AdjCatId::ColorationBrillante,
                    AdjCatId::ColorationBleue],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::Plante,
            NounCategory {
                id: NounCatId::Plante,
                nouns: vec![
                    NounId::Mousse,
                    NounId::Liane,
                    NounId::Lierre,
                    NounId::Chevrefeuille,
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::Liberte,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse
                    ],
                    epithets: vec![
                        AdjCatId::Liberte,
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Anonymat,
                    ],
                    functions: vec![],
                    emissions: vec![NounCatId::PhenomeneOlfactif],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::PlanteAFleur,
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
                    attributes: vec![AdjCatId::EtatDeFloraison, AdjCatId::Liberte],
                    epithets: vec![
                        AdjCatId::EtatDeFloraison,
                        AdjCatId::Liberte,
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationRousse,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::Anonymat,
                        AdjCatId::Saveur,
                    ],
                    functions: vec![],
                    emissions: vec![NounCatId::PhenomeneOlfactif],
                    affiliations: vec![NounCatId::MomentDuJour],
                },
            },
        ),
        (
            NounCatId::OrganeDePlante,
            NounCategory {
                id: NounCatId::OrganeDePlante,
                nouns: vec![
                    NounId::Feuille,
                    NounId::Branche,
                    NounId::Petale,
                    NounId::Etamine,
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                    ],
                    epithets: vec![
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Saveur,
                    ],
                    functions: vec![],
                    emissions: vec![
                        NounCatId::PhenomeneSonoreFloral,
                        NounCatId::PhenomeneOlfactif
                    ],
                    affiliations: vec![NounCatId::PlanteAFleur],
                },
            },
        ),
        (
            NounCatId::MomentDuJour,
            NounCategory {
                id: NounCatId::MomentDuJour,
                nouns: vec![NounId::Aurore, NounId::Crepuscule,],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Chaleur,
                        AdjCatId::Froideur,
                        AdjCatId::Calme,
                    ],
                    epithets: vec![
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Chaleur,
                        AdjCatId::Froideur,
                        AdjCatId::Calme,
                    ],
                    functions: vec![],
                    emissions: vec![
                        NounCatId::PhenomeneLumineux,
                        NounCatId::PhenomeneOlfactif,
                        NounCatId::Phenomene
                    ],
                    affiliations: vec![NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::MiJour,
            NounCategory {
                id: NounCatId::MiJour,
                nouns: vec![NounId::Midi, NounId::Minuit],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![AdjCatId::Chaleur, AdjCatId::Froideur, AdjCatId::Calme,                         AdjCatId::ColorationTexture,
                    AdjCatId::ColorationDiapree,
                    AdjCatId::ColorationBrillante,
                    AdjCatId::ColorationBleue,
                    AdjCatId::ColorationRousse,
                    AdjCatId::ColorationTexture,
                    ],
                    functions: vec![],
                    emissions: vec![
                        NounCatId::PhenomeneLumineux,
                        NounCatId::PhenomeneOlfactif,
                        NounCatId::Phenomene
                    ],
                    affiliations: vec![NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::Oiseau,
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
                    NounId::Pie,
                ],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::Calme,
                        AdjCatId::Agitation,
                        AdjCatId::CaractereMoqueur
                    ],
                    epithets: vec![
                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationSombre,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::Noblesse,
                        AdjCatId::CaractereMoqueur,
                        AdjCatId::Anonymat,
                    ],
                    functions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![NounCatId::PhenomeneSonore],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::Mammifere,
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
                    attributes: vec![
                        AdjCatId::Calme,
                        AdjCatId::Agitation,
                        AdjCatId::CaractereMoqueur
                    ],
                    epithets: vec![

                        AdjCatId::ColorationBrillante,
                        AdjCatId::ColorationRousse,
                        AdjCatId::Noblesse,
                        AdjCatId::Grandeur,
                        AdjCatId::CaractereMoqueur,
                        AdjCatId::Anonymat,
                    ],
                    functions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![],
                    affiliations: vec![],
                },
            },
        ),
        (
            NounCatId::PartieDOiseau,
            NounCategory {
                id: NounCatId::PartieDOiseau,
                nouns: vec![NounId::Plume,/* NounId::Duvet,*/],
                rel: NounRelations {
                    attributes: vec![
                        AdjCatId::ColorationBrillante
                    ],
                    epithets: vec![
                        AdjCatId::Delicatesse,
                        AdjCatId::ColorationSombre,
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::Anonymat,
                    ],
                    functions: vec![],
                    emissions: vec![NounCatId::EffetLumineux],
                    affiliations: vec![NounCatId::Oiseau],
                },
            },
        ),
        (
            NounCatId::EffetLumineux,
            NounCategory {
                id: NounCatId::EffetLumineux,
                nouns: vec![NounId::Reflet, NounId::Eclat, NounId::Raie,],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![
                        AdjCatId::ColorationDiapree,
                        AdjCatId::ColorationBleue,
                        AdjCatId::ColorationRousse,
                        AdjCatId::ColorationTexture,
                        AdjCatId::ColorationBrillante,
                        AdjCatId::Noblesse,
                    ],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::Astre],
                },
            },
        ),
        (
            NounCatId::PortionDeTemps,
            NounCategory {
                id: NounCatId::PortionDeTemps,
                nouns: vec![NounId::Moment, NounId::Heure, NounId::Minute,],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![AdjCatId::Noblesse, AdjCatId::ColorationTexture,
                    AdjCatId::ColorationBrillante],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![
                        NounCatId::Calme,
                        NounCatId::Astre,
                        NounCatId::PhenomeneMeteo
                    ],
                },
            },
        ),
        (
            NounCatId::Calme,
            NounCategory {
                id: NounCatId::Calme,
                nouns: vec![NounId::Paix, NounId::Calme, NounId::Serenite],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::PhenomeneMeteo,
            NounCategory {
                id: NounCatId::PhenomeneMeteo,
                nouns: vec![
                    NounId::Pluie,
                    NounId::Bruine,
                    NounId::Orage,
                    NounId::Averse,
                    NounId::Embellie,
                    NounId::Brume,
                ],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![],
                    functions: vec![],
                    emissions: vec![],
                    affiliations: vec![NounCatId::Saison],
                },
            },
        ),
        (
            NounCatId::EspritHumain,
            NounCategory {
                id: NounCatId::EspritHumain,
                nouns: vec![
                    NounId::Ame,
                    NounId::Coeur,
                    NounId::Esprit,
                ],
                rel: NounRelations {
                    attributes: vec![],
                    epithets: vec![AdjCatId::Calme, AdjCatId::Delicatesse],
                    functions: vec![VerbCatId::EtatDEveil],
                    emissions: vec![],
                    affiliations: vec![NounCatId::Saison],
                },
            },
        )
    ]
    .iter()
    .cloned()
    .collect();
}
