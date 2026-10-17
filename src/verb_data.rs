use std::collections::HashMap;
use std::sync::LazyLock;

use crate::verb;
use crate::verb_enums;
use verb::Verb;
use verb_enums::{VerbCatId, VerbGroup, VerbId, VerbKind};

use crate::wordgroup;
use wordgroup::WordGroup;

pub type StaticVerbs = [Verb; 13];
pub static VERBS: LazyLock<StaticVerbs> = LazyLock::new(|| {
    [
        Verb::new_special(
            VerbId::Etre,
            "être",
            (1, 1),
            false,
            VerbKind::Copule,
            Some(WordGroup::new("suis", 1)),
            Some(WordGroup::new("est", 1)),
            Some(WordGroup::new("sont", 1)),
        ),
        Verb::new_special(
            VerbId::Paraitre,
            "paraître",
            (2, 2),
            false,
            VerbKind::Copule,
            Some(WordGroup::new("parais", 2)),
            Some(WordGroup::new("paraît", 2)),
            Some(WordGroup::new("paraissent", 2)),
        ),
        Verb::new_special(
            VerbId::AvoirLAir,
            "avoir l'air",
            (1, 1),
            false,
            VerbKind::Copule,
            Some(WordGroup::new("ai l'air", 2)),
            Some(WordGroup::new("a l'air", 2)),
            Some(WordGroup::new("ont l'air", 2)),
        ),
        Verb::new(
            VerbId::Rester,
            "rester",
            (2, 2),
            VerbGroup::First,
            false,
            VerbKind::Copule,
        ),
        Verb::new(
            VerbId::Sembler,
            "sembler",
            (2, 2),
            VerbGroup::First,
            false,
            VerbKind::Copule,
        ),
        Verb::new(
            VerbId::Demeurer,
            "demeurer",
            (3, 3),
            VerbGroup::First,
            false,
            VerbKind::Copule,
        ),
        Verb::new(
            VerbId::SAssoupir,
            "assoupir",
            (3, 3),
            VerbGroup::Second,
            true,
            VerbKind::Intransitive,
        ),
        Verb::new(
            VerbId::SEvanouir,
            "évanouir",
            (4, 4),
            VerbGroup::Second,
            true,
            VerbKind::Intransitive,
        ),
        Verb::new(
            VerbId::SEveiller,
            "éveiller",
            (3, 3),
            VerbGroup::First,
            true,
            VerbKind::Intransitive,
        ),
        Verb::new_special(
            VerbId::SEndormir,
            "endormir",
            (3, 3),
            true,
            VerbKind::Intransitive,
            Some(WordGroup::new("endors", 2)),
            Some(WordGroup::new("endort", 2)),
            Some(WordGroup::new("endorment", 2)),
        ),
        Verb::new(
            VerbId::SEclipser,
            "éclipser",
            (3, 3),
            VerbGroup::First,
            true,
            VerbKind::Intransitive,
        ),
        Verb::new(
            VerbId::SeCoucher,
            "coucher",
            (2, 2),
            VerbGroup::First,
            true,
            VerbKind::Intransitive,
        ),
        Verb::new(
            VerbId::SeLever,
            "lever",
            (2, 2),
            VerbGroup::First,
            true,
            VerbKind::Intransitive,
        ),
    ]
});

pub type VerbCatHashMap = HashMap<VerbCatId, Vec<VerbId>>;
pub static VERB_CATS: LazyLock<VerbCatHashMap> = LazyLock::new(|| {
    [
        (
            VerbCatId::EtatDEveil,
            vec![
                VerbId::SAssoupir,
                VerbId::SEveiller,
                VerbId::SEclipser,
                VerbId::SeCoucher,
                VerbId::SeLever,
                VerbId::SEvanouir,
                VerbId::SEndormir,
            ],
        ),
        (
            VerbCatId::Etat,
            vec![
                VerbId::Etre,
                VerbId::Paraitre,
                VerbId::AvoirLAir,
                VerbId::Rester,
                VerbId::Sembler,
                VerbId::Demeurer,
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect()
});
