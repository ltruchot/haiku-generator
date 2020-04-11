use std::collections::HashMap;

use crate::adj;
use crate::adj_enums;
use adj::Adj;
use adj_enums::{AdjCatId, AdjId};

pub type StaticAdjs = [Adj; 39];
lazy_static! {
    pub static ref ADJS: StaticAdjs = [
        Adj::new_special(AdjId::EnFleur, "en fleur", None, None, None, true, (2, 2)),
        Adj::new(AdjId::Libre, "libre", (1, 2)),
        Adj::new(AdjId::Sauvage, "sauvage", (2, 3)),
        Adj::new(AdjId::Printanier, "printanier", (3, 4)),
        Adj::new(AdjId::Estival, "estival", (3, 3)),
        Adj::new(AdjId::Automnal, "automnal", (3, 3)),
        Adj::new(AdjId::Hivernal, "hivernal", (3, 3)),
        Adj::new(AdjId::Violet, "violet", (2, 3)),
        Adj::new(AdjId::Orange, "orange", (2, 3)),
        Adj::new(AdjId::Brun, "brun", (1, 1)),
        Adj::new(AdjId::Dore, "doré", (2, 2)),
        Adj::new(AdjId::Argente, "argenté", (3, 3)),
        Adj::new_special(
            AdjId::Roux,
            "roux",
            Some(String::from("rousse")),
            Some(String::from("roux")),
            Some(String::from("rousses")),
            false,
            (1, 2)
        ),
        Adj::new(AdjId::Ecarlate, "écarlate", (3, 4)),
        // Adj::new(AdjId::Grand, "grand", (1, 1)),
        Adj::new(AdjId::Immense, "immense", (2, 3)),
        Adj::new(AdjId::Majestueux, "majestueux", (4, 4)),
        Adj::new(AdjId::Noble, "noble", (1, 2)),
        Adj::new(AdjId::Delicat, "délicat", (3, 3)),

        // agitation
        Adj::new_special(AdjId::Inquiet, "inquiet", Some(String::from("inquiète")), None, None, false, (2, 3)),
        Adj::new_special(AdjId::AuxAbois, "aux abois", None, None, None, true, (3,3)),
        Adj::new(AdjId::Vigilant, "vigilant", (3, 3)),
        Adj::new(AdjId::Perplexe, "perplexe", (3, 4)),
        Adj::new(AdjId::Agite, "agité", (3, 3)),
        Adj::new(AdjId::Emu, "ému", (2, 2)),

        // calme
        Adj::new(AdjId::Calme, "calme", (1, 2)),
        Adj::new(AdjId::Sage, "sage", (1, 2)),
        Adj::new(AdjId::Heureux, "heureux", (2, 2)),
        Adj::new(AdjId::Immobile, "immobile", (3, 4)),
        Adj::new_special(AdjId::EnPaix, "en paix", None, None, None, true, (2, 2)),

        // chaleur
        Adj::new(AdjId::Caniculaire, "caniculaire", (4, 4)),
        Adj::new(AdjId::Irradiant, "irradiant", (3, 4)),
        Adj::new(AdjId::Aride, "aride", (2, 3)),

        // froideur
        Adj::new(AdjId::Polaire, "polaire", (2, 3)),
        Adj::new(AdjId::Glacial, "glacial", (2, 3)),
        Adj::new(AdjId::Glace, "glacé", (2, 2)),

        // caractère 
        Adj::new(AdjId::Moqueur, "moqueur", (2, 2)),
        Adj::new(AdjId::Malicieux, "malicieux", (3, 4)),
        Adj::new(AdjId::Narquois, "narquois", (2, 2)),
        Adj::new(AdjId::Railleur, "railleur", (2, 2)),
        //Adj::new(AdjId::Farouche, "farouche", (2, 3)),
    ];
}

pub type AdjCatHashMap = HashMap<AdjCatId, Vec<AdjId>>;
lazy_static! {
    pub static ref ADJ_CATS: AdjCatHashMap = [
        (AdjCatId::EtatDeFloraison, vec![AdjId::EnFleur,],),
        (AdjCatId::Liberte, vec![AdjId::Libre, AdjId::Sauvage],),
        (
            AdjCatId::RelAUneSaison,
            vec![
                AdjId::Printanier,
                AdjId::Estival,
                AdjId::Automnal,
                AdjId::Hivernal,
            ],
        ),
        (
            AdjCatId::Coloration,
            vec![
                AdjId::Violet,
                AdjId::Orange,
                AdjId::Brun,
                AdjId::Dore,
                AdjId::Argente,
            ],
        ),
        (
            AdjCatId::ColorationRousse,
            vec![
                AdjId::Dore,
                AdjId::Orange,
                AdjId::Roux,
                AdjId::Ecarlate,
                AdjId::Brun,
            ],
        ),
        (
            AdjCatId::Grandeur,
            vec![AdjId::Grand, AdjId::Immense, AdjId::Majestueux,]
        ),
        (
            AdjCatId::Noblesse,
            vec![AdjId::Noble, AdjId::Delicat, AdjId::Majestueux,]
        ),
        (
            AdjCatId::Agitation,
            vec![
                AdjId::Inquiet,
                AdjId::AuxAbois,
                AdjId::Vigilant,
                AdjId::Perplexe,
                AdjId::Agite,
                AdjId::Emu,
            ]
        ),
        (
            AdjCatId::Calme,
            vec![
                AdjId::Calme,
                AdjId::Sage,
                AdjId::Heureux,
                AdjId::Immobile,
                AdjId::EnPaix,
            ]
        ),
        (
            AdjCatId::Chaleur,
            vec![
                AdjId::Caniculaire,
                AdjId::Irradiant,
                AdjId::Aride,
            ]
        ),
        (
            AdjCatId::Froideur,
            vec![
                AdjId::Polaire,
                AdjId::Glacial,
                AdjId::Glace
            ]
        ),
        (
            AdjCatId::CaractereMoqueur,
            vec![
                AdjId::Moqueur,
                AdjId::Malicieux,
                AdjId::Narquois,
                AdjId::Railleur,
            ]
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
