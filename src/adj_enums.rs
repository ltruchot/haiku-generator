#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjCatId {
    EtatDeFloraison,
    Liberte,
    RelAUneSaison,
    ColorationTexture,
    ColorationDiapree,
    ColorationBleue,
    ColorationRousse,
    ColorationSombre,
    ColorationBrillante,
    Grandeur,
    Noblesse,
    Delicatesse,
    Agitation,
    Calme,
    Chaleur,
    Froideur,
    CaractereMoqueur,
    Anonymat,
    Saveur,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjId {
    // divers
    EnFleur,
    Sauvage,
    Libre,

    // saison
    Printanier,
    Estival,
    Automnal,
    Hivernal,

    // couleur
    Violet,
    Orange,
    Brun,
    Dore,
    Argente,
    Roux,
    Ecarlate,
    Ocre,
    Pastel,
    Bleu,
    Rose,

    // coloration diaphane
    Diaphane,
    Diapre,
    Moire,
    Irise,
    Hale,
    Bigarre,

    // taille
    Grand,
    Immense,

    // noblesse
    Majestueux,
    Noble,
    Delicat,

    // agitation
    Inquiet,
    AuxAbois,
    Vigilant,
    Perplexe,
    Agite,
    Emu,

    // calme
    Calme,
    Sage,
    Heureux,
    Immobile,
    EnPaix,

    // chaleur
    Caniculaire,
    /* Irradiant, */
    Brulant,

    // froideur
    Glace,
    Glacial,
    Polaire,

    // caractère moqueur
    Moqueur,
    Malicieux,
    Narquois,
    Railleur,

    // coloration sombre
    Noir,
    Sombre,
    Gris,
    Ebene,

    // anonymat
    Anonyme,
    Etrange,
    Etranger,

    // saveur
    Epice,
    Sucre,


}
