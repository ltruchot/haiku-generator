#[derive(Copy, Clone)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Copy, Clone)]
pub enum Number {
    Plural,
    Singular,
}

#[derive(Copy, Clone)]
pub enum Article {
    Definite,
    Indefinite,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum AdjCatId {
    EnFleur,
    Sauvage,
    RelAUneSaison,
    Couleur,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum NounCatId {
    Astre,
    Phenomene,
    PhenomeneLumineux,
    PhenomeneSonore,
    PhenomeneOlfactif,
    Saison,
    PlanteAFleur,
    OrganeDePlante,
    MomentDuJour,
    Plante,
    Oiseau,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum NounId {
    Lune,
    Soleil,
    Etoile,
    Bruit,
    Chant,
    Bruissement,
    Lumiere,
    Rayon,
    Odeur,
    Parfum,
    Arome,
    Printemps,
    Ete,
    Automne,
    Hiver,
    Prunier,
    Cerisier,
    Oeillet,
    Glycine,
    Pivoine,
    Feuille,
    Branche,
    Aurore,
    Crepuscule,
    Midi,
    Minuit,
    Mousse,
    Liane,
    Lierre,
    Chevrefeuille,
    Petale,
    Etamine,
    Alouette,
    Mesange,
    Grive,
    Canard,
    RougeGorge,
    Fauvette,
    Hirondelle,
    Merle,
    Pic,
}
// le soleil s'éclipse
