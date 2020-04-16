#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbGroup {
    First,
    Second,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum VerbKind {
    /* Transitive, */
    Intransitive,
    /* Labile, */
    Copule,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbCatId {
    Etat,
    EtatDEveil,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbId {
    // Etat
    Sembler,
    Demeurer,
    Rester,

    // État d'éveil
    SAssoupir,
    SEveiller,
    SEclipser,
    SeCoucher,
    SeLever,
    SEvanouir,
}
