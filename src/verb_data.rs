use std::collections::HashMap;

use crate::verb;
use crate::verb_enums;
use verb::Verb;
use verb_enums::{VerbGroup, VerbCatId, VerbId};

pub type StaticVerbs = [Verb; 6];
lazy_static! {
    pub static ref VERBS: StaticVerbs = [
        Verb::new(VerbId::SAssoupir, "assoupir", (3, 3), VerbGroup::Second, true),
        Verb::new(VerbId::SEvanouir, "évanouir", (3, 3), VerbGroup::Second, true),
        Verb::new(VerbId::SEveiller, "éveiller", (3, 3), VerbGroup::First, true),
        Verb::new(VerbId::SEclipser, "éclipser", (3, 3), VerbGroup::First, true),
        Verb::new(VerbId::SeCoucher, "coucher", (2, 2), VerbGroup::First, true),
        Verb::new(VerbId::SeLever, "lever", (2, 2), VerbGroup::First, true),
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
    ]
    .iter()
    .cloned()
    .collect();
}
