use std::collections::HashMap;

use crate::adj;
use crate::adj_enums;
use adj::Adj;
use adj_enums::{AdjCatId, AdjId};

pub type StaticAdjs = [Adj; 19];
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
        Adj::new(AdjId::Brun, "brun", (2, 3)),
        Adj::new(AdjId::Dore, "doré", (3, 3)),
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
        Adj::new(AdjId::Grand, "grand", (1, 1)),
        Adj::new(AdjId::Immense, "immense", (2, 3)),
        Adj::new(AdjId::Majestueux, "majestueux", (4, 4)),
        Adj::new(AdjId::Noble, "noble", (1, 2)),
        Adj::new(AdjId::Delicat, "délicat", (3, 3)),
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
    ]
    .iter()
    .cloned()
    .collect();
}
