use std::collections::HashMap;

use crate::verb;
use crate::verb_enums;
use verb::Verb;
use verb_enums::{VerbGroup, VerbCatId, VerbId, VerbKind};

pub type StaticVerbs = [Verb; 9];
lazy_static! {
    pub static ref VERBS: StaticVerbs = [
        Verb::new(VerbId::Rester, "rester", (2, 2), VerbGroup::First, false, VerbKind::Copule),
        Verb::new(VerbId::Sembler, "sembler", (2, 2), VerbGroup::First, false, VerbKind::Copule),
        Verb::new(VerbId::Demeurer, "demeurer", (3, 3), VerbGroup::First, false, VerbKind::Copule),
        Verb::new(VerbId::SAssoupir, "assoupir", (3, 3), VerbGroup::Second, true, VerbKind::Intransitive),
        Verb::new(VerbId::SEvanouir, "évanouir", (4, 4), VerbGroup::Second, true, VerbKind::Intransitive),
        Verb::new(VerbId::SEveiller, "éveiller", (3, 3), VerbGroup::First, true, VerbKind::Intransitive),
        Verb::new(VerbId::SEclipser, "éclipser", (3, 3), VerbGroup::First, true, VerbKind::Intransitive),
        Verb::new(VerbId::SeCoucher, "coucher", (2, 2), VerbGroup::First, true, VerbKind::Intransitive),
        Verb::new(VerbId::SeLever, "lever", (2, 2), VerbGroup::First, true, VerbKind::Intransitive),
    ];
}

pub type VerbCatHashMap = HashMap<VerbCatId, Vec<VerbId>>;
lazy_static! {
    pub static ref VERB_CATS: VerbCatHashMap = [
        (
            VerbCatId::EtatDEveil,
            vec![
                VerbId::SAssoupir,
                VerbId::SEveiller,
                VerbId::SEclipser,
                VerbId::SeCoucher,
                VerbId::SeLever,
                VerbId::SEvanouir,
            ],
        ),
        (
            VerbCatId::Etat,
            vec![
                VerbId::Rester,
                VerbId::Sembler,
                VerbId::Demeurer,
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
