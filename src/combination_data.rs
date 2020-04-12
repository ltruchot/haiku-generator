// IMPORTS

// commons
use crate::common_enums;
use common_enums::Article;

use crate::combination;
use combination::{
    get_with_adj_and_affiliation, get_with_adjective, get_with_affiliation,
    get_with_intransitive_verb, get_with_linking_verb, Combination,
};

// EXPORTS
pub type Combinations = [Combination; 5];

pub fn get_combinations() -> Combinations {
    [
        get_with_adjective(None, None),
        get_with_linking_verb(None, Some(vec![Article::Definite])),
        get_with_intransitive_verb(None, Some(vec![Article::Definite])),
        get_with_affiliation(None, None),
        get_with_adj_and_affiliation(None, None),
    ]
}
