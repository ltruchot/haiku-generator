#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjCatId {
    EnFleur,
    Sauvage,
    RelAUneSaison,
    Couleur,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjId {
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

    // divers
    EnFleur,
    Sauvage,
}
