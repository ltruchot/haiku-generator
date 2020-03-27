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
pub enum AdjId {
    EnFleur,
    Sauvage,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum CategoryId {
    Astre,
    Phenomene,
    Saison,
    PlanteAFleur,
    OrganeDePlante,
}