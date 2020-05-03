use std::collections::HashMap;

use crate::adj;
use crate::adj_enums;
use adj::Adj;
use adj_enums::{AdjCatId, AdjId};

pub type StaticAdjs = [Adj; 59];
lazy_static! {
    pub static ref ADJS: StaticAdjs = [
        Adj::new_special(AdjId::EnFleur, "en fleur", None, None, None, true, (2, 2)),
        Adj::new(AdjId::Libre, "libre", (1, 1)),
        Adj::new(AdjId::Sauvage, "sauvage", (2, 2)),
        Adj::new(AdjId::Printanier, "printanier", (3, 4)),
        Adj::new(AdjId::Estival, "estival", (3, 3)),
        Adj::new(AdjId::Automnal, "automnal", (3, 3)),
        Adj::new(AdjId::Hivernal, "hivernal", (3, 3)),
        Adj::new(AdjId::Violet, "violet", (2, 3)),
        Adj::new(AdjId::Orange, "orange", (2, 2)),
        Adj::new(AdjId::Ocre, "ocre", (1, 1)),
        Adj::new_special(AdjId::Pastel, "pastel", None, None, None, true, (2,2)),
        Adj::new(AdjId::Brun, "brun", (1, 1)),
        Adj::new(AdjId::Bleu, "bleu", (1, 1)),
        Adj::new(AdjId::Dore, "doré", (2, 2)),
        Adj::new(AdjId::Argente, "argenté", (3, 3)),
        Adj::new_special(
            AdjId::Roux,
            "roux",
            Some(String::from("rousse")),
            Some(String::from("roux")),
            Some(String::from("rousses")),
            false,
            (1, 1)
        ),
        Adj::new(AdjId::Ecarlate, "écarlate", (3, 3)),
        // Adj::new(AdjId::Grand, "grand", (1, 1)),
        Adj::new(AdjId::Immense, "immense", (2, 2)),
        Adj::new(AdjId::Majestueux, "majestueux", (4, 4)),
        Adj::new(AdjId::Noble, "noble", (1, 1)),
        Adj::new(AdjId::Delicat, "délicat", (3, 3)),

        // agitation
        Adj::new_special(AdjId::Inquiet, "inquiet", Some(String::from("inquiète")), None, Some(String::from("inquiètes")), false, (2, 3)),
        Adj::new_special(AdjId::AuxAbois, "aux abois", None, None, None, true, (3,3)),
        Adj::new(AdjId::Vigilant, "vigilant", (3, 3)),
        Adj::new(AdjId::Perplexe, "perplexe", (3, 3)),
        Adj::new(AdjId::Agite, "agité", (3, 3)),
        Adj::new(AdjId::Emu, "ému", (2, 2)),

        // calme
        Adj::new(AdjId::Calme, "calme", (1, 1)),
        Adj::new(AdjId::Sage, "sage", (1, 1)),
        Adj::new(AdjId::Heureux, "heureux", (2, 2)),
        Adj::new(AdjId::Immobile, "immobile", (3, 3)),
        Adj::new_special(AdjId::EnPaix, "en paix", None, None, None, true, (2, 2)),

        // chaleur
        Adj::new(AdjId::Caniculaire, "caniculaire", (4, 4)),
        // Adj::new(AdjId::Irradiant, "irradiant", (3, 4)),
        Adj::new(AdjId::Brulant, "brûlant", (2, 2)),

        // froideur
        Adj::new(AdjId::Polaire, "polaire", (2, 2)),
        Adj::new_special(AdjId::Glacial, "glacial", None, Some(String::from("glacials")), None, false, (2, 3)),
        Adj::new(AdjId::Glace, "glacé", (2, 2)),

        // caractère
        Adj::new(AdjId::Moqueur, "moqueur", (2, 2)),
        Adj::new(AdjId::Malicieux, "malicieux", (3, 4)),
        Adj::new(AdjId::Narquois, "narquois", (2, 2)),
        Adj::new(AdjId::Railleur, "railleur", (2, 2)),
        //Adj::new(AdjId::Farouche, "farouche", (2, 3)),

        // coloration sombre
        Adj::new(AdjId::Noir, "noir", (1, 1)),
        Adj::new(AdjId::Sombre, "sombre", (1, 1)),
        Adj::new(AdjId::Gris, "gris", (1, 1)),
        Adj::new(AdjId::Ebene, "ébène", (2, 2)),

        // coloration diaprée
        Adj::new(AdjId::Diapre, "diapré", (2, 3)),
        Adj::new(AdjId::Moire, "moiré", (2, 2)),
        Adj::new(AdjId::Irise, "irisé", (3, 3)),
        Adj::new(AdjId::Hale, "halé", (2, 2)),
        Adj::new(AdjId::Hale, "bigarré", (3, 3)),

        // anonymat
        Adj::new(AdjId::Anonyme, "anonyme", (3, 3)),
        Adj::new(AdjId::Etrange, "étrange", (2, 2)),
        Adj::new(AdjId::Etranger, "étranger", (3, 3)),

        // saveur
        Adj::new(AdjId::Epice, "épicé", (3,3)),
        Adj::new(AdjId::Sucre, "sucré", (2, 2)),
        Adj::new(AdjId::Rose, "rose", (1, 1)),
        Adj::new(AdjId::Paisible, "paisible", (2,2)),
        Adj::new(AdjId::Leger, "léger", (2, 2)),
        Adj::new(AdjId::Paisible, "doux", (1,1))
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
            AdjCatId::ColorationTexture,
            vec![
                AdjId::Pastel,
            ],
        ),
        (
            AdjCatId::ColorationBleue,
            vec![
                AdjId::Bleu,
                AdjId::Violet,
            ],
        ),
        (
            AdjCatId::ColorationBrillante,
            vec![
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
                AdjId::Ocre,
                AdjId::Violet,
                AdjId::Rose,
            ],
        ),
        (
            AdjCatId::ColorationSombre,
            vec![
                AdjId::Noir,
                AdjId::Sombre,
                AdjId::Gris,
                AdjId::Ebene,
            ]
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
                AdjId::Paisible,
            ]
        ),
        (
            AdjCatId::Chaleur,
            vec![AdjId::Caniculaire, /*AdjId::Irradiant,*/AdjId::Brulant,]
        ),
        (
            AdjCatId::Froideur,
            vec![AdjId::Polaire, AdjId::Glacial, AdjId::Glace]
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
        (
            AdjCatId::ColorationDiapree,
            vec![
                AdjId::Diaphane,
                AdjId::Diapre,
                AdjId::Moire,
                AdjId::Irise,
                AdjId::Hale,
                AdjId::Bigarre,
            ]
        ),
        (
            AdjCatId::Delicatesse,
            vec![
                AdjId::Delicat,
                AdjId::Noble,
                AdjId::Leger,
                AdjId::Doux,
            ]
        ),
        (
            AdjCatId::Anonymat,
            vec![
                AdjId::Anonyme,
                AdjId::Etrange,
                AdjId::Etranger,
            ]
        ),
        (
            AdjCatId::Saveur,
            vec![
                AdjId::Sucre,
                AdjId::Epice,
            ]
        )

    ]
    .iter()
    .cloned()
    .collect();
}
