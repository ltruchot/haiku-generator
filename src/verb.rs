#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbCatId {
    EtatDEveil
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum VerbId {
    SEndormir,
    SEveiller,
    SEclipser,
    SeCoucher,
    SeLever,
}
