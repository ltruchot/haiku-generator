// Exhaustive enumeration of what each sentence pattern can produce when it is
// called with EMPTY blacklists: the REAL closures of `combination.rs`, driven
// through every path of their decision tree by the odometer.
//
// Besides the lines themselves it gives two things a sample does not: the
// probability of every outcome (used to size the Monte-Carlo runs) and which
// lemmas a line is made of (the blacklists the closure returns; used by the
// haiku properties).
//
// The set of outcomes does not depend on the iteration order of the category
// HashMaps; the order in which the odometer meets them does, which is why the
// goldens are sorted sets and not path -> line tables.

use std::collections::BTreeMap;

use crate::combination_data::get_combinations;
use crate::common_enums::BlackLists;
use crate::wordgroup::WordGroup;

use super::labels::{find_adj, find_noun, find_verb};
use super::odometer::walk;

/// Same order as `combination_data::get_combinations()`.
pub const PATTERN_NAMES: [&str; 5] = [
    "adjective",
    "linking_verb",
    "intransitive_verb",
    "affiliation",
    "adjective_and_affiliation",
];

pub struct Outcome {
    /// The golden line: `text | min max`, or `#ERR# first + second`.
    pub line: String,
    /// `None` when the pattern returns `Err`.
    pub word: Option<WordGroup>,
    // lemmas the line is made of, sorted
    pub nouns: Vec<&'static str>,
    pub adjs: Vec<&'static str>,
    pub verbs: Vec<&'static str>,
    /// Probability of this path of the decision tree.
    pub prob: f64,
}

pub fn ok_line(word: &WordGroup) -> String {
    format!("{} | {} {}", word.text, word.foots.0, word.foots.1)
}

pub fn err_line(errs: &[String]) -> String {
    format!("#ERR# {}", errs.join(" + "))
}

fn sorted(mut lemmas: Vec<&'static str>) -> Vec<&'static str> {
    lemmas.sort();
    lemmas
}

/// Every path of the decision tree of pattern `index` (see PATTERN_NAMES).
pub fn enumerate(index: usize) -> Vec<Outcome> {
    let patterns = get_combinations();
    let empty = BlackLists::new_empty();
    walk(|odometer| patterns[index](odometer, &empty))
        .into_iter()
        .map(|(result, prob, _)| match result {
            Ok((word, used)) => Outcome {
                line: ok_line(&word),
                word: Some(word),
                // a word that was used has an entry in its table
                nouns: sorted(
                    used.nouns
                        .iter()
                        .map(|id| find_noun(*id).unwrap().word.text.as_str())
                        .collect(),
                ),
                adjs: sorted(
                    used.adjs
                        .iter()
                        .map(|id| find_adj(*id).unwrap().word.text.as_str())
                        .collect(),
                ),
                verbs: sorted(
                    used.verbs
                        .iter()
                        .map(|id| find_verb(*id).unwrap().word.text.as_str())
                        .collect(),
                ),
                prob,
            },
            Err(errs) => Outcome {
                line: err_line(&errs),
                word: None,
                nouns: vec![],
                adjs: vec![],
                verbs: vec![],
                prob,
            },
        })
        .collect()
}

/// Distinct golden lines with their total probability, sorted.
pub fn distribution(outcomes: &[Outcome]) -> BTreeMap<String, f64> {
    let mut lines: BTreeMap<String, f64> = BTreeMap::new();
    for outcome in outcomes.iter() {
        *lines.entry(outcome.line.clone()).or_insert(0.0) += outcome.prob;
    }
    lines
}
