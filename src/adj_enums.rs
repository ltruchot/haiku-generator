#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjCatId {
    EtatDeFloraison,
    Liberte,
    RelAUneSaison,
    Coloration,
    ColorationRousse,
    Grandeur,
    Noblesse,
    Agitation,
    Calme,
    Chaleur,
    Froideur,
    CaractereMoqueur,
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
    Aride,

    // froideur
    Glace,
    Glacial,
    Polaire,

    // caractère moqueur
    Moqueur,
    Malicieux,
    Narquois,
    Railleur,
}
