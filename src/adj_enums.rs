#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjCatId {
    EtatDeFloraison,
    Liberte,
    RelAUneSaison,
    Coloration,
    ColorationRousse,
    Grandeur,
    Noblesse,
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
}
