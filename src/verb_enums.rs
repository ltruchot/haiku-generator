#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbGroup {
    First,
    Second,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbCatId {
    EtatDEveil
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbId {
    SAssoupir,
    SEveiller,
    SEclipser,
    SeCoucher,
    SeLever,
    SEvanouir,
}
